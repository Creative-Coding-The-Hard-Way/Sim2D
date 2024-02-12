mod api;
mod color_pass;
mod pipeline;
mod transform;
mod vertex_buffer;

use {
    self::{
        color_pass::ColorPass,
        pipeline::{GraphicsPipeline, PushConstants},
        transform::Transform,
        vertex_buffer::VertexBuffer,
    },
    super::Renderer,
    crate::{
        graphics::vulkan::{
            frames_in_flight::{
                BeginFrameStatus, EndFrameStatus, FramesInFlight,
            },
            render_context::RenderContext,
            swapchain::Swapchain,
            sync::AsyncNBuffer,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::{sync::mpsc::Receiver, time::Instant},
};

pub use self::{
    api::InterpolatedPrimitivesApi,
    vertex_buffer::{Vertex, WritableVertexBuffer},
};

/// Parameters for the renderer.
#[derive(Debug, Copy, Clone)]
pub struct Parameters {
    /// The topology for the vertices. Defaults to TRIANGLE_LIST.
    pub topology: vk::PrimitiveTopology,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        }
    }
}

/// A renderer which supports rendering colored primitive geometry like points,
/// lines, and triangles.
///
/// Of note: this renderer does not support textures.
///
/// Additionally, each vertex can have an associated velocity. The vertex
/// position is approximated based on the velocity and the time since last
/// update. This makes frames appear smooth even if updates happen less than
/// once per frame.
pub struct InterpolatedPrimitivesRenderer {
    rc: RenderContext,

    swapchain: Swapchain,
    color_pass: ColorPass,
    pipeline: GraphicsPipeline,
    swapchain_needs_rebuild: bool,
    framebuffer_size: (u32, u32),

    framebuffer_size_reciever: Receiver<(u32, u32)>,
    vertices: AsyncNBuffer<VertexBuffer>,
    transform: AsyncNBuffer<Transform>,
    frames_in_flight: FramesInFlight,
}

impl Renderer for InterpolatedPrimitivesRenderer {
    type Api = InterpolatedPrimitivesApi;
    type Parameters = Parameters;

    /// Create a new Renderer and client api.
    fn new(
        rc: &RenderContext,
        parameters: Self::Parameters,
    ) -> Result<(Self, Self::Api)>
    where
        Self: Sized,
    {
        let swapchain = Swapchain::new(rc, (1, 1))
            .with_context(trace!("Unable to create the swapchain!"))?;
        let color_pass = ColorPass::new(rc, &swapchain)
            .with_context(trace!("Unable to create the color pass!"))?;
        let pipeline = GraphicsPipeline::new(
            rc,
            &color_pass.render_pass,
            parameters.topology,
        )
        .with_context(trace!("Unable to create the graphics pipeline!"))?;
        let frames_in_flight = FramesInFlight::new(rc, 2)
            .with_context(trace!("Unable to create frames in flight!"))?;
        let (vertices, vertices_client) =
            VertexBuffer::create_n_buffered(rc, frames_in_flight.count() + 1)
                .with_context(trace!("Unable to create streamable vertices!"))?;
        let (transform, transform_client) = Transform::create_n_buffered(
            rc,
            pipeline.descriptor_set_layout.clone(),
            2,
        )
        .with_context(trace!("Unable to create transform buffers!"))?;

        let (framebuffer_size_sender, framebuffer_size_reciever) =
            std::sync::mpsc::channel::<(u32, u32)>();

        Ok((
            Self {
                rc: rc.clone(),
                swapchain,
                color_pass,
                pipeline,
                swapchain_needs_rebuild: false,
                framebuffer_size: (1, 1),
                frames_in_flight,
                framebuffer_size_reciever,
                vertices,
                transform,
            },
            Self::Api::new(
                vertices_client,
                transform_client,
                framebuffer_size_sender,
            ),
        ))
    }

    /// Render a single frame.
    ///
    /// Automatically rebuilds the swapchain and processes any messages from the
    /// client api.
    fn draw_frame(&mut self) -> Result<()> {
        // Rebuild the Swapchain if needed
        if self.swapchain_needs_rebuild {
            // pull in any framebuffer updates
            while let Ok(framebuffer_size) =
                self.framebuffer_size_reciever.try_recv()
            {
                self.framebuffer_size = framebuffer_size;
            }
            return self.rebuild_swapchain(self.framebuffer_size);
        }

        self.present_frame()?;

        Ok(())
    }

    /// Shut down the renderer. Wait for every frame to finish and stall the
    /// GPU in anticipation of dropping all Vulkan resources.
    fn shut_down(&mut self) -> Result<()> {
        log::info!("wait for all frames to finish");
        unsafe {
            self.frames_in_flight.wait_for_all_frames(&self.rc)?;
            self.rc.device.device_wait_idle()?;
        }
        Ok(())
    }
}

impl InterpolatedPrimitivesRenderer {
    /// Rebuild the swapchain and dependent resources
    fn rebuild_swapchain(
        &mut self,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        // finish all frames in flight before rebuilding
        self.frames_in_flight.wait_for_all_frames(&self.rc)?;
        self.vertices.free_all()?; // since all frames are now finished
        self.transform.free_all()?; // since all frames are now finished

        // rebuild the swapchain
        unsafe {
            self.framebuffer_size = framebuffer_size;
            self.swapchain
                .rebuild_swapchain(&self.rc, self.framebuffer_size)
                .with_context(trace!("Unable to resize the swapchain!"))?
        };

        // Rebuild swapchain-dependent resources
        self.color_pass = ColorPass::new(&self.rc, &self.swapchain)
            .with_context(trace!("Unable to rebuild the color pass!"))?;
        self.pipeline = GraphicsPipeline::new(
            &self.rc,
            &self.color_pass.render_pass,
            self.pipeline.topology,
        )
        .with_context(trace!("Unable to rebuild the graphics pipeline!"))?;

        self.swapchain_needs_rebuild = false;
        Ok(())
    }

    /// Acquire a swapchain image, get the current frame in flight, and
    /// prepare the frame's rendering commands.
    pub fn present_frame(&mut self) -> Result<()> {
        let start = Instant::now();
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
        let got_frame = Instant::now();

        let current_frame_index = self.frames_in_flight.current_frame_index();

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

        // Bind the descriptor set
        {
            let transform = self.transform.get_current(current_frame_index)?;
            unsafe {
                self.rc.device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline.pipeline_layout.raw,
                    0,
                    &[transform.descriptor_set],
                    &[],
                );
            }
        }

        let (vertex_buffer, last_update) = self
            .vertices
            .get_current_with_update_time(current_frame_index)?;
        // Set push constants
        {
            let dt = (Instant::now() - last_update).as_secs_f32();
            let constants = PushConstants {
                dt,
                vertex_buffer_addr: vertex_buffer.buffer_address,
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
                self.rc.device.cmd_draw(
                    command_buffer,
                    vertex_buffer.vertex_count,
                    1,
                    0,
                    0,
                );
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

        let dt = (Instant::now() - start).as_secs_f32();
        let acquire_delay = (got_frame - start).as_secs_f32();
        log::trace!(
            indoc::indoc! {"
                render time: {}ms
                acquire_frame_time: {}ms
            "},
            (dt * 100_000.0).round() / 100.0,
            (acquire_delay * 1_000_000.0).round() / 1000.0,
        );

        Ok(())
    }
}
