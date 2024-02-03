mod frame;

use {
    super::swapchain::PresentImageStatus,
    crate::{
        graphics::vulkan::{
            render_context::RenderContext,
            swapchain::{AcquireImageStatus, Swapchain},
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    frame::Frame,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BeginFrameStatus {
    Acquired(vk::CommandBuffer),
    SwapchainNeedsRebuild,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EndFrameStatus {
    Finished,
    SwapchainNeedsRebuild,
}

/// This struct is responsible for all of the resources required to synchronize
/// multiple frames in flight.
pub struct FramesInFlight {
    frames: Vec<Frame>,
    current_frame_index: usize,
    swapchain_image_index: u32,
}

impl FramesInFlight {
    /// Create synchronization primitives for multiple frames in flight.
    pub fn new(rc: &RenderContext, count: usize) -> Result<Self> {
        let mut frames = Vec::with_capacity(count);
        for i in 0..count {
            frames.push(
                Frame::new(rc)
                    .with_context(trace!("Error while creating frame {}", i))?,
            );
        }
        Ok(Self {
            frames,
            current_frame_index: 0,
            swapchain_image_index: 0,
        })
    }

    /// Get the index of the current swapchain image
    pub fn swapchain_image_index(&self) -> usize {
        self.swapchain_image_index as usize
    }

    /// Get the index of the frame that is currently in-flight.
    pub fn current_frame_index(&self) -> usize {
        self.current_frame_index
    }

    /// Get the command buffer for the frame that is currently in-flight.
    pub fn command_buffer(&self) -> vk::CommandBuffer {
        self.current_frame().command_buffer
    }

    /// The maximum total frames in flight.
    pub fn count(&self) -> usize {
        self.frames.len()
    }

    /// Begin the next frame in flight.
    pub fn begin_frame(
        &mut self,
        rc: &RenderContext,
        swapchain: &Swapchain,
    ) -> Result<BeginFrameStatus> {
        // Update the current frame index
        self.current_frame_index =
            (self.current_frame_index + 1) % self.frames.len();

        // Make sure the frame's last command submission has finished
        unsafe {
            rc.device
                .wait_for_fences(
                    &[*self.current_frame().graphics_commands_finished],
                    true,
                    std::u64::MAX,
                )
                .with_context(trace!(
                    "Error while waiting for frame {} to finish!",
                    self.current_frame_index
                ))?;
        }

        // Acquire the next swapchain image
        self.swapchain_image_index = {
            let result = swapchain
                .acquire_swapchain_image(
                    *self.current_frame().swapchain_image_available,
                )
                .with_context(trace!(
                    "Unable to get the next swapchain image!"
                ))?;
            match result {
                AcquireImageStatus::NeedsRebuild => {
                    return Ok(BeginFrameStatus::SwapchainNeedsRebuild);
                }
                AcquireImageStatus::ImageAcequired(index) => index,
            }
        };

        // Reset Frame resources
        unsafe {
            rc.device
                .reset_fences(&[*self
                    .current_frame()
                    .graphics_commands_finished])
                .with_context(trace!("Unable to reset frame graphics fence"))?;
            rc.device
                .reset_command_pool(
                    *self.current_frame().command_pool,
                    vk::CommandPoolResetFlags::empty(),
                )
                .with_context(trace!("Unable to reset frame command pool"))?;
        }

        // Begin the frame's command buffer
        {
            let begin_info = vk::CommandBufferBeginInfo {
                flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                ..Default::default()
            };
            unsafe {
                rc.device
                    .begin_command_buffer(
                        self.current_frame().command_buffer,
                        &begin_info,
                    )
                    .with_context(trace!(
                        "Unable to begin the frame's graphics commands!"
                    ))?;
            }
        }

        Ok(BeginFrameStatus::Acquired(
            self.current_frame().command_buffer,
        ))
    }

    /// End the current frame.
    pub fn end_frame(
        &mut self,
        rc: &RenderContext,
        swapchain: &Swapchain,
    ) -> Result<EndFrameStatus> {
        // end the command buffer
        unsafe {
            rc.device
                .end_command_buffer(self.current_frame().command_buffer)
                .with_context(trace!(
                    "Unable to end the frame's command buffer!"
                ))?
        };

        // Submit the graphics commands
        {
            let wait_stage = vk::PipelineStageFlags::VERTEX_SHADER;
            let submit = vk::SubmitInfo {
                wait_semaphore_count: 1,
                p_wait_semaphores: &self
                    .current_frame()
                    .swapchain_image_available
                    .raw,
                p_wait_dst_stage_mask: &wait_stage,
                command_buffer_count: 1,
                p_command_buffers: &self.current_frame().command_buffer,
                signal_semaphore_count: 1,
                p_signal_semaphores: &self
                    .current_frame()
                    .graphics_submission_finished
                    .raw,
                ..Default::default()
            };
            unsafe {
                rc.device
                    .queue_submit(
                        rc.graphics_queue,
                        &[submit],
                        self.current_frame().graphics_commands_finished.raw,
                    )
                    .with_context(trace!(
                        "Unable to submit graphics commands for frame!"
                    ))?;
            }
        }

        // Present the swapchain image
        {
            let result = swapchain
                .present_swapchain_image(
                    rc,
                    self.current_frame().graphics_submission_finished.raw,
                    self.swapchain_image_index,
                )
                .with_context(trace!(
                    "Error while presenting swapchain image {}!",
                    self.swapchain_image_index
                ))?;
            if result == PresentImageStatus::NeedsRebuild {
                return Ok(EndFrameStatus::SwapchainNeedsRebuild);
            }
        }

        Ok(EndFrameStatus::Finished)
    }

    /// Block until all graphics commands for every frame have finished
    /// executing on the GPU.
    pub fn wait_for_all_frames(&self, rc: &RenderContext) -> Result<()> {
        let fences: Vec<vk::Fence> = self
            .frames
            .iter()
            .map(|frame| *frame.graphics_commands_finished)
            .collect();
        unsafe {
            rc.device
                .wait_for_fences(&fences, true, std::u64::MAX)
                .with_context(trace!(
                    "Error while waiting for frames to finish!"
                ))?;
        }
        Ok(())
    }

    fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame_index]
    }
}
