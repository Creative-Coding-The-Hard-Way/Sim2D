mod pipeline;
mod render_pass;
mod streamable_vertices;

use {
    anyhow::{Context, Result},
    ash::vk,
    pipeline::{GraphicsPipeline, PushConstants},
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::vulkan::{
            frames_in_flight::{
                BeginFrameStatus, EndFrameStatus, FramesInFlight,
            },
            memory::DeviceAllocator,
            render_context::{Instance, RenderContext, Surface},
            swapchain::Swapchain,
        },
    },
    std::time::Instant,
    streamable_vertices::{StreamableVerticies, Vertex},
};

struct MyApp {
    rc: RenderContext,
    swapchain: Swapchain,
    pipeline: pipeline::GraphicsPipeline,
    color_pass: render_pass::ColorPass,
    swapchain_needs_rebuild: bool,
    framebuffer_size: (u32, u32),
    frames_in_flight: FramesInFlight,
    start_time: std::time::Instant,
    vertices: StreamableVerticies,
}

impl MyApp {
    /// Rebuild the swapchain and dependent resources
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The swapchain and dependent resources must not be in use when they are
    ///   rebuilt.
    unsafe fn rebuild_swapchain(&mut self) -> Result<()> {
        // destroy swapchain dependent resources
        self.color_pass.destroy(&self.rc);
        self.pipeline.destroy(&self.rc);

        // rebuild the swapchain
        self.swapchain
            .rebuild_swapchain(&self.rc, self.framebuffer_size)
            .with_context(|| "Unable to resize the swapchain!")?;

        // Rebuild swapchain-dependent resources
        self.color_pass =
            render_pass::ColorPass::new(&self.rc, &self.swapchain)?;
        self.pipeline =
            GraphicsPipeline::new(&self.rc, &self.color_pass.render_pass)
                .with_context(|| "Unable to rebuild the graphics pipeline!")?;
        Ok(())
    }
}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title("Example 01");

        let instance = Instance::new(
            "Example 01",
            &window
                .glfw
                .get_required_instance_extensions()
                .unwrap_or_default(),
        )?;
        log::info!("Vulkan Instance created! \n{:#?}", instance);

        let rc = RenderContext::new(
            instance.clone(),
            Surface::from_glfw_window(window, &instance)?,
        )?;
        let allocator = DeviceAllocator::new(&rc);

        let (w, h) = window.get_framebuffer_size();
        let swapchain = Swapchain::new(&rc, (w as u32, h as u32))?;

        let color_pass = render_pass::ColorPass::new(&rc, &swapchain)?;
        let pipeline =
            pipeline::GraphicsPipeline::new(&rc, &color_pass.render_pass)?;

        let frames_in_flight = FramesInFlight::new(&rc, 3)?;
        let vertices = StreamableVerticies::new(&rc, &allocator, 2)?;

        Ok(MyApp {
            rc,
            swapchain,
            pipeline,
            color_pass,
            swapchain_needs_rebuild: false,
            framebuffer_size: (w as u32, h as u32),
            frames_in_flight,
            start_time: Instant::now(),
            vertices,
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        if let &glfw::WindowEvent::FramebufferSize(width, height) = event {
            self.swapchain_needs_rebuild = true;
            self.framebuffer_size = (width as u32, height as u32);
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
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

        {
            if let Some(mut vertex_buffer) =
                self.vertices.try_get_writable_buffer()
            {
                let t = Instant::now()
                    .duration_since(self.start_time)
                    .as_secs_f32();
                let a0 = t * std::f32::consts::TAU / 10.0;
                let a1 = a0 + std::f32::consts::TAU / 3.0;
                let a2 = a1 + std::f32::consts::TAU / 3.0;
                let r = 0.75;
                unsafe {
                    vertex_buffer.write_vertex_data(&[
                        Vertex {
                            rgba: [1.0, 0.0, 0.0, 1.0],
                            pos: [r * a0.cos(), r * a0.sin()],
                            ..Default::default()
                        },
                        Vertex {
                            rgba: [0.0, 1.0, 0.0, 1.0],
                            pos: [r * a1.cos(), r * a1.sin()],
                            ..Default::default()
                        },
                        Vertex {
                            rgba: [0.0, 0.0, 1.0, 1.0],
                            pos: [r * a2.cos(), r * a2.sin()],
                            ..Default::default()
                        },
                    ]);
                }
                self.vertices.publish_update(vertex_buffer);
                log::info!(
                    "Frame {}: got buffer",
                    self.frames_in_flight.current_frame_index()
                );
            } else {
                log::info!(
                    "Frame {}: no buffer available!",
                    self.frames_in_flight.current_frame_index()
                );
            }
        }

        // Begin the render pass
        {
            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            };
            let render_pass_begin = vk::RenderPassBeginInfo {
                render_pass: self.color_pass.render_pass,
                framebuffer: self.color_pass.framebuffers
                    [self.frames_in_flight.swapchain_image_index()],
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
                    self.pipeline.handle,
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
                    self.pipeline.pipeline_layout,
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

    fn destroy(&mut self) -> Result<()> {
        unsafe {
            // Wait for all operations to finish.
            self.frames_in_flight.wait_for_all_frames(&self.rc)?;
            self.rc.device.device_wait_idle()?;

            // Destroy everything.
            self.vertices.destroy(&self.rc);
            self.frames_in_flight.destroy(&self.rc);
            self.color_pass.destroy(&self.rc);
            self.pipeline.destroy(&self.rc);
            self.swapchain.destroy(&self.rc);
            self.rc.destroy();
        };
        Ok(())
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
