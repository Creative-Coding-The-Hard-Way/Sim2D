mod pipeline;
mod render_pass;

use {
    anyhow::{Context, Result},
    ash::vk,
    pipeline::GraphicsPipeline,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::vulkan::{
            render_context::{Instance, RenderContext, Surface},
            swapchain::{AcquireImageStatus, PresentImageStatus, Swapchain},
        },
    },
};

struct MyApp {
    rc: RenderContext,
    swapchain: Swapchain,
    pipeline: pipeline::GraphicsPipeline,
    color_pass: render_pass::ColorPass,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    image_available_semaphore: vk::Semaphore,
    render_finished_semaphore: vk::Semaphore,
    in_flight_fence: vk::Fence,
    swapchain_needs_rebuild: bool,
    framebuffer_size: (u32, u32),
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

        let (w, h) = window.get_framebuffer_size();
        let swapchain = Swapchain::new(&rc, (w as u32, h as u32))?;

        let color_pass = render_pass::ColorPass::new(&rc, &swapchain)?;
        let pipeline =
            pipeline::GraphicsPipeline::new(&rc, &color_pass.render_pass)?;

        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo {
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: rc.graphics_queue_index,
                ..Default::default()
            };
            unsafe { rc.device.create_command_pool(&create_info, None)? }
        };
        let command_buffer = {
            let allocate_info = vk::CommandBufferAllocateInfo {
                command_pool,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
                ..Default::default()
            };
            unsafe { rc.device.allocate_command_buffers(&allocate_info)?[0] }
        };
        let image_available_semaphore = {
            let create_info = vk::SemaphoreCreateInfo::default();
            unsafe { rc.device.create_semaphore(&create_info, None)? }
        };
        let render_finished_semaphore = {
            let create_info = vk::SemaphoreCreateInfo::default();
            unsafe { rc.device.create_semaphore(&create_info, None)? }
        };
        let in_flight_fence = {
            let create_info = vk::FenceCreateInfo {
                flags: vk::FenceCreateFlags::SIGNALED,
                ..Default::default()
            };
            unsafe { rc.device.create_fence(&create_info, None)? }
        };

        Ok(MyApp {
            rc,
            swapchain,
            pipeline,
            color_pass,
            command_pool,
            command_buffer,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
            swapchain_needs_rebuild: false,
            framebuffer_size: (w as u32, h as u32),
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
                self.rc.device.device_wait_idle()?;
                self.rebuild_swapchain()?
            };
            self.swapchain_needs_rebuild = false;
        }

        // Wait for the last frame to finish
        unsafe {
            self.rc.device.wait_for_fences(
                &[self.in_flight_fence],
                true,
                std::u64::MAX,
            )?
        }

        // Acquire the next swapchain image
        let image_index = {
            let result = self
                .swapchain
                .acquire_swapchain_image(self.image_available_semaphore)?;
            match result {
                AcquireImageStatus::NeedsRebuild => {
                    self.swapchain_needs_rebuild = true;
                    return Ok(());
                }
                AcquireImageStatus::ImageAcequired(index) => index,
            }
        };

        // Reset the buffer
        unsafe {
            self.rc.device.reset_fences(&[self.in_flight_fence])?;
            self.rc.device.reset_command_pool(
                self.command_pool,
                vk::CommandPoolResetFlags::empty(),
            )?;
        }

        // Begin the command buffer
        {
            let begin_info = vk::CommandBufferBeginInfo {
                flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                ..Default::default()
            };
            unsafe {
                self.rc
                    .device
                    .begin_command_buffer(self.command_buffer, &begin_info)?;
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
                framebuffer: self.color_pass.framebuffers[image_index as usize],
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
                    self.command_buffer,
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
                self.rc.device.cmd_set_viewport(
                    self.command_buffer,
                    0,
                    &viewports,
                );
            }
        }

        // Set the scissor region
        {
            let scissors = [vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: self.swapchain.extent,
            }];
            unsafe {
                self.rc.device.cmd_set_scissor(
                    self.command_buffer,
                    0,
                    &scissors,
                );
            }
        }

        // Bind the graphics pipeline
        {
            unsafe {
                self.rc.device.cmd_bind_pipeline(
                    self.command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline.handle,
                );
            }
        }

        // Draw!
        {
            unsafe {
                self.rc.device.cmd_draw(self.command_buffer, 3, 1, 0, 0);
            }
        }

        // End the render pass
        {
            unsafe {
                self.rc.device.cmd_end_render_pass(self.command_buffer);
            }
        }

        // end the command buffer
        unsafe { self.rc.device.end_command_buffer(self.command_buffer)? };

        // Submit the graphics commands
        {
            let wait_stage = vk::PipelineStageFlags::VERTEX_SHADER;
            let submit = vk::SubmitInfo {
                wait_semaphore_count: 1,
                p_wait_semaphores: &self.image_available_semaphore,
                p_wait_dst_stage_mask: &wait_stage,
                command_buffer_count: 1,
                p_command_buffers: &self.command_buffer,
                signal_semaphore_count: 1,
                p_signal_semaphores: &self.render_finished_semaphore,
                ..Default::default()
            };
            unsafe {
                self.rc.device.queue_submit(
                    self.rc.graphics_queue,
                    &[submit],
                    self.in_flight_fence,
                )?;
            }
        }

        // Present the image
        {
            let result = self
                .swapchain
                .present_swapchain_image(
                    &self.rc,
                    self.render_finished_semaphore,
                    image_index,
                )
                .context("Error while presenting the image!")?;
            if result == PresentImageStatus::NeedsRebuild {
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        }

        Ok(())
    }

    fn destroy(&mut self) -> Result<()> {
        unsafe {
            // Wait for all operations to finish.
            self.rc.device.device_wait_idle()?;

            // Destroy everything.
            self.rc.device.destroy_fence(self.in_flight_fence, None);
            self.rc
                .device
                .destroy_semaphore(self.image_available_semaphore, None);
            self.rc
                .device
                .destroy_semaphore(self.render_finished_semaphore, None);
            self.rc.device.destroy_command_pool(self.command_pool, None);
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
