use {
    crate::{
        graphics::vulkan::{raii, render_context::RenderContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// All of the resources which are allocated per-frame for presentation.
pub struct Frame {
    /// A semaphore that is signalled when the swapchain image is available.
    pub swapchain_image_available: raii::Semaphore,

    /// A semaphore that is signalled when the frame's graphics command
    /// submission is finished.
    pub graphics_submission_finished: raii::Semaphore,

    /// A fence that is signalled when the frame's graphics commands are
    /// finished executing on the GPU.
    pub graphics_commands_finished: raii::Fence,

    /// A command pool for the frame.
    pub command_pool: raii::CommandPool,

    /// The Frame's primary graphics command buffer. Reset every frame.
    pub command_buffer: vk::CommandBuffer,
}

impl Frame {
    pub fn new(rc: &RenderContext) -> Result<Self> {
        let swapchain_image_available = {
            raii::Semaphore::new_single_owner(
                rc.device.clone(),
                &vk::SemaphoreCreateInfo::default(),
            )
            .with_context(trace!("Unable to create semaphore!"))?
        };
        let graphics_submission_finished = {
            raii::Semaphore::new_single_owner(
                rc.device.clone(),
                &vk::SemaphoreCreateInfo::default(),
            )
            .with_context(trace!("Unable to create semaphore!"))?
        };
        let graphics_commands_finished = {
            let create_info = vk::FenceCreateInfo {
                flags: vk::FenceCreateFlags::SIGNALED,
                ..Default::default()
            };
            raii::Fence::new_single_owner(rc.device.clone(), &create_info)
                .with_context(trace!("Unable to create fence!"))?
        };
        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo {
                flags: vk::CommandPoolCreateFlags::TRANSIENT,
                queue_family_index: rc.graphics_queue_index,
                ..Default::default()
            };
            raii::CommandPool::new_single_owner(rc.device.clone(), &create_info)
                .with_context(trace!("Unable to create command pool"))?
        };
        let command_buffer = {
            let allocate_info = vk::CommandBufferAllocateInfo {
                command_pool: *command_pool,
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
}
