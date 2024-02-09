mod create;
mod image_views;

use {
    crate::{
        graphics::vulkan::{raii, render_context::RenderContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Returned when acquiring a swapchain image.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AcquireImageStatus {
    /// Indicates that the swapchain image with a given index was acquired.
    ImageAcequired(u32),

    /// Indicates that the swapchain needs rebuilt.
    NeedsRebuild,
}

/// Returned when presenting a swapchain image.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PresentImageStatus {
    /// Indicates that the image was queued for presentation.
    Queued,

    /// Indicates ethat the swapchain needs rebuilt.
    NeedsRebuild,
}

/// The application swapchain and associated resources.
pub struct Swapchain {
    pub extent: vk::Extent2D,
    pub surface_format: vk::SurfaceFormatKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<raii::ImageViewArc>,
    pub swapchain: raii::SwapchainArc,
    pub surface: raii::SurfaceArc,
}

impl Swapchain {
    /// Create a new swapchain.
    pub fn new(
        rc: &RenderContext,
        framebuffer_size: (u32, u32),
    ) -> Result<Self> {
        let (swapchain, extent, surface_format) =
            create::create_swapchain(rc, framebuffer_size, None)
                .with_context(trace!("Unable to initialize the swapchain!"))?;
        let images = unsafe {
            swapchain
                .ext
                .get_swapchain_images(swapchain.raw)
                .with_context(trace!("Unable to get swapchain images!"))?
        };
        let image_views =
            image_views::create_image_views(rc, &images, surface_format.format)
                .with_context(trace!(
                    "Unable to create swapchain image views!"
                ))?;
        Ok(Self {
            surface: rc.surface.clone(),
            extent,
            surface_format,
            images,
            image_views,
            swapchain,
        })
    }

    /// Acquire the next swapchain image.
    ///
    /// # Params
    ///
    /// - `image_available_semaphore`: A semaphore to signal when the swapchain
    ///   image is available for rendering.
    ///
    /// # Returns
    ///
    /// An enum containing either the next swapchain index or a signal to
    /// rebuild the swapchain.
    pub fn acquire_swapchain_image(
        &self,
        image_available_semaphore: vk::Semaphore,
    ) -> Result<AcquireImageStatus> {
        let result = unsafe {
            self.swapchain.ext.acquire_next_image(
                self.swapchain.raw,
                std::u64::MAX,
                image_available_semaphore,
                vk::Fence::null(),
            )
        };
        match result {
            Ok((index, false)) => {
                // Image acquired and not suboptimal
                Ok(AcquireImageStatus::ImageAcequired(index))
            }
            Ok((index, true)) => {
                // Image acquired but the swapchain is suboptimal
                log::warn!("Swapchain is suboptimal! Request a rebuild...");
                Ok(AcquireImageStatus::ImageAcequired(index))
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                Ok(AcquireImageStatus::NeedsRebuild)
            }
            Err(error) => Err(error).with_context(trace!(
                "Error while acquiring the swapchain image!"
            )),
        }
    }

    /// Present A Swapchain image
    ///
    /// # Params
    ///
    /// - `wait_semaphore`: The semaphore to wait on before presenting.
    pub fn present_swapchain_image(
        &self,
        rc: &RenderContext,
        wait_semaphore: vk::Semaphore,
        image_index: u32,
    ) -> Result<PresentImageStatus> {
        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: &wait_semaphore,
            swapchain_count: 1,
            p_swapchains: &self.swapchain.raw,
            p_image_indices: &image_index,
            ..Default::default()
        };
        let result = unsafe {
            self.swapchain
                .ext
                .queue_present(rc.present_queue, &present_info)
        };
        match result {
            Ok(false) => Ok(PresentImageStatus::Queued),
            Ok(true) => Ok(PresentImageStatus::NeedsRebuild),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                Ok(PresentImageStatus::NeedsRebuild)
            }
            Err(err) => Err(err).with_context(trace!(
                "Unable to present swapchain image {}",
                image_index
            )),
        }
    }

    /// Rebuild the swapchain.
    ///
    /// Typically called when the application window is resized or the display
    /// format changes.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The caller must ensure that the swapchain is not being used by the GPU
    ///   when this method is called. (typically via fences for frames in flight
    ///   or by waiting for the device to idle etc...).
    pub unsafe fn rebuild_swapchain(
        &mut self,
        rc: &RenderContext,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        let (swapchain, extent, surface_format) = create::create_swapchain(
            rc,
            framebuffer_size,
            Some(self.swapchain.raw),
        )
        .with_context(trace!("Unable to rebuild the swapchain!"))?;

        self.swapchain = swapchain;
        self.extent = extent;
        self.surface_format = surface_format;

        self.images = unsafe {
            self.swapchain
                .ext
                .get_swapchain_images(self.swapchain.raw)
                .with_context(trace!("Unable to get swapchain images!"))?
        };
        self.image_views = image_views::create_image_views(
            rc,
            &self.images,
            self.surface_format.format,
        )
        .with_context(trace!("Unable to create swapchain image views!"))?;

        Ok(())
    }
}
