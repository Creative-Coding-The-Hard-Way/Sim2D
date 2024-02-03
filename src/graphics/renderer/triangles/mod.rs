mod color_pass;
mod pipeline;
mod streamable_vertices;
mod vertex;

use {
    self::{
        color_pass::ColorPass,
        pipeline::{GraphicsPipeline, PushConstants},
        streamable_vertices::StreamableVerticies,
    },
    crate::{
        graphics::vulkan::{
            frames_in_flight::{
                BeginFrameStatus, EndFrameStatus, FramesInFlight,
            },
            render_context::RenderContext,
            swapchain::Swapchain,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

pub use self::{streamable_vertices::VertexBuffer, vertex::Vertex};

pub type IsRunning = std::sync::Arc<std::sync::atomic::AtomicBool>;

/// A renderer for streaming colored triangles.
pub struct Triangles {
    rc: RenderContext,

    swapchain: Swapchain,
    color_pass: ColorPass,
    pipeline: GraphicsPipeline,
    swapchain_needs_rebuild: bool,
    framebuffer_size: (u32, u32),

    vertices: StreamableVerticies,
    frames_in_flight: FramesInFlight,
}

impl Triangles {
    pub fn new(
        rc: RenderContext,
        framebuffer_size: (u32, u32),
    ) -> Result<Self> {
        let swapchain = Swapchain::new(&rc, framebuffer_size)
            .with_context(trace!("Unable to create the swapchain!"))?;
        let color_pass = ColorPass::new(&rc, &swapchain)
            .with_context(trace!("Unable to create the color pass!"))?;
        let pipeline = GraphicsPipeline::new(&rc, &color_pass.render_pass)
            .with_context(trace!("Unable to create the graphics pipeline!"))?;
        let frames_in_flight = FramesInFlight::new(&rc, 2)
            .with_context(trace!("Unable to create frames in flight!"))?;
        let mut vertices =
            StreamableVerticies::new(&rc, frames_in_flight.count() + 1)
                .with_context(trace!(
                    "Unable to create streamable vertices!"
                ))?;

        unsafe {
            let mut vertex_buffer = vertices.try_get_writable_buffer().unwrap();
            vertex_buffer.write_vertex_data(&[
                Vertex::new(0.0, 0.0, 1.0, 0.0, 0.0, 1.0),
                Vertex::new(0.0, 0.0, 1.0, 0.0, 0.0, 1.0),
                Vertex::new(0.0, 0.0, 1.0, 0.0, 0.0, 1.0),
            ]);
            vertices.publish_update(vertex_buffer);
        }

        Ok(Self {
            rc,
            swapchain,
            color_pass,
            pipeline,
            swapchain_needs_rebuild: false,
            framebuffer_size,
            frames_in_flight,
            vertices,
        })
    }

    /// Rebuild the swapchain and dependent resources
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The swapchain and dependent resources must not be in use when they are
    ///   rebuilt.
    pub unsafe fn rebuild_swapchain(&mut self) -> Result<()> {
        // rebuild the swapchain
        self.swapchain
            .rebuild_swapchain(&self.rc, self.framebuffer_size)
            .with_context(trace!("Unable to resize the swapchain!"))?;

        // Rebuild swapchain-dependent resources
        self.color_pass = ColorPass::new(&self.rc, &self.swapchain)
            .with_context(trace!("Unable to rebuild the color pass!"))?;
        self.pipeline =
            GraphicsPipeline::new(&self.rc, &self.color_pass.render_pass)
                .with_context(trace!(
                    "Unable to rebuild the graphics pipeline!"
                ))?;
        Ok(())
    }

    pub fn try_get_writable_buffer(&mut self) -> Option<VertexBuffer> {
        self.vertices.try_get_writable_buffer()
    }

    pub fn publish_update(&mut self, vertices: VertexBuffer) {
        self.vertices.publish_update(vertices);
    }

    pub fn draw(&mut self) -> Result<()> {
        // Rebuild the Swapchain if needed
        if self.swapchain_needs_rebuild {
            unsafe {
                self.frames_in_flight.wait_for_all_frames(&self.rc)?;
                self.rebuild_swapchain()?
            };
            self.swapchain_needs_rebuild = false;
        }

        let command_buffer = match self
            .frames_in_flight
            .begin_frame(&self.rc, &self.swapchain)?
        {
            BeginFrameStatus::Acquired(command_buffer) => command_buffer,
            BeginFrameStatus::SwapchainNeedsRebuild => {
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        };

        // Begin the render pass
        {
            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            };
            let render_pass_begin = vk::RenderPassBeginInfo {
                render_pass: self.color_pass.render_pass.raw,
                framebuffer: self.color_pass.framebuffers
                    [self.frames_in_flight.swapchain_image_index()]
                .raw,
                render_area: vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: self.swapchain.extent,
                },
                clear_value_count: 1,
                p_clear_values: &clear_value,
                ..Default::default()
            };
            unsafe {
                self.rc.device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin,
                    vk::SubpassContents::INLINE,
                );
            }
        }

        // Set the viewport
        {
            let viewports = [vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: self.swapchain.extent.width as f32,
                height: self.swapchain.extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }];
            unsafe {
                self.rc
                    .device
                    .cmd_set_viewport(command_buffer, 0, &viewports);
            }
        }

        // Set the scissor region
        {
            let scissors = [vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: self.swapchain.extent,
            }];
            unsafe {
                self.rc.device.cmd_set_scissor(command_buffer, 0, &scissors);
            }
        }

        // Bind the graphics pipeline
        {
            unsafe {
                self.rc.device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline.pipeline.raw,
                );
            }
        }

        // Set push constants
        {
            let constants = PushConstants {
                vertex_buffer_addr: self
                    .vertices
                    .get_read_buffer(
                        self.frames_in_flight.current_frame_index(),
                    )
                    .device_buffer_addr,
            };
            unsafe {
                let constants_ptr = std::slice::from_raw_parts(
                    &constants as *const PushConstants as *const u8,
                    std::mem::size_of::<PushConstants>(),
                );
                self.rc.device.cmd_push_constants(
                    command_buffer,
                    self.pipeline.pipeline_layout.raw,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    constants_ptr,
                );
            }
        }

        // Draw!
        {
            unsafe {
                self.rc.device.cmd_draw(command_buffer, 3, 1, 0, 0);
            }
        }

        // End the render pass
        {
            unsafe {
                self.rc.device.cmd_end_render_pass(command_buffer);
            }
        }

        if self.frames_in_flight.end_frame(&self.rc, &self.swapchain)?
            == EndFrameStatus::SwapchainNeedsRebuild
        {
            self.swapchain_needs_rebuild = true;
            return Ok(());
        }

        Ok(())
    }

    pub fn destroy(&mut self) -> Result<()> {
        unsafe {
            // stall everything before destroying
            self.frames_in_flight.wait_for_all_frames(&self.rc)?;
            self.rc.device.device_wait_idle()?;

            //
            self.vertices.destroy(&self.rc);
        }
        Ok(())
    }
}
