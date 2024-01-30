use {
    crate::{graphics::vulkan::render_context::RenderContext, trace},
    anyhow::{Context, Result},
    ash::vk,
};

/// All of the resources which are allocated per-frame for presentation.
#[derive(Clone, Copy)]
pub struct Frame {
    /// A semaphore that is signalled when the swapchain image is available.
    pub swapchain_image_available: vk::Semaphore,

    /// A semaphore that is signalled when the frame's graphics command
    /// submission is finished.
    pub graphics_submission_finished: vk::Semaphore,

    /// A fence that is signalled when the frame's graphics commands are
    /// finished executing on the GPU.
    pub graphics_commands_finished: vk::Fence,

    /// A command pool for the frame.
    pub command_pool: vk::CommandPool,

    /// The Frame's primary graphics command buffer. Reset every frame.
    pub command_buffer: vk::CommandBuffer,
}

impl Frame {
    pub fn new(rc: &RenderContext) -> Result<Self> {
        let swapchain_image_available = {
            let create_info = vk::SemaphoreCreateInfo::default();
            unsafe {
                rc.device
                    .create_semaphore(&create_info, None)
                    .with_context(trace!("Unable to create semaphore!"))?
            }
        };
        let graphics_submission_finished = {
            let create_info = vk::SemaphoreCreateInfo::default();
            unsafe {
                rc.device
                    .create_semaphore(&create_info, None)
                    .with_context(trace!("Unable to create semaphore!"))?
            }
        };
        let graphics_commands_finished = {
            let create_info = vk::FenceCreateInfo {
                flags: vk::FenceCreateFlags::SIGNALED,
                ..Default::default()
            };
            unsafe {
                rc.device
                    .create_fence(&create_info, None)
                    .with_context(trace!("Unable to create fence!"))?
            }
        };
        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo {
                flags: vk::CommandPoolCreateFlags::TRANSIENT,
                queue_family_index: rc.graphics_queue_index,
                ..Default::default()
            };
            unsafe {
                rc.device
                    .create_command_pool(&create_info, None)
                    .with_context(trace!("Unable to create command pool"))?
            }
        };
        let command_buffer = {
            let allocate_info = vk::CommandBufferAllocateInfo {
                command_pool,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
                ..Default::default()
            };
            unsafe {
                rc.device
                    .allocate_command_buffers(&allocate_info)
                    .with_context(trace!("Unable to allocate command buffer"))?
                    [0]
            }
        };
        Ok(Self {
            swapchain_image_available,
            graphics_submission_finished,
            graphics_commands_finished,
            command_pool,
            command_buffer,
        })
    }

    /// Destroy all frame resources.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - this method must not be called until the graphics command finished
    ///   fence has been signalled.
    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        rc.device
            .destroy_semaphore(self.swapchain_image_available, None);
        rc.device
            .destroy_semaphore(self.graphics_submission_finished, None);
        rc.device
            .destroy_fence(self.graphics_commands_finished, None);
        rc.device.destroy_command_pool(self.command_pool, None);
    }
}
