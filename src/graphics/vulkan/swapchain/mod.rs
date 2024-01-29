mod create;
mod image_views;

use {
    super::render_context::RenderContext,
    crate::trace,
    anyhow::{Context, Result},
    ash::vk,
};

/// Returned when acquiring a swapchain image.
pub enum AcquireImageStatus {
    /// Indicates that the swapchain image with a given index was acquired.
    ImageAcequired(u32),

    /// Indicates that the swapchain needs rebuilt.
    NeedsRebuild,
}

/// The application swapchain and associated resources.
#[derive(Clone)]
pub struct Swapchain {
    pub handle: vk::SwapchainKHR,
    pub loader: ash::extensions::khr::Swapchain,
    pub extent: vk::Extent2D,
    pub surface_format: vk::SurfaceFormatKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
}

impl Swapchain {
    /// Create a new swapchain.
    pub fn new(
        rc: &RenderContext,
        framebuffer_size: (u32, u32),
    ) -> Result<Self> {
        let loader =
            ash::extensions::khr::Swapchain::new(&rc.instance.ash, &rc.device);
        let (handle, extent, surface_format) =
            create::create_swapchain(rc, &loader, framebuffer_size, None)
                .with_context(trace!("Unable to initialize the swapchain!"))?;
        let images = unsafe {
            loader
                .get_swapchain_images(handle)
                .with_context(trace!("Unable to get swapchain images!"))?
        };
        let image_views =
            image_views::create_image_views(rc, &images, surface_format.format)
                .with_context(trace!(
                    "Unable to create swapchain image views!"
                ))?;
        Ok(Self {
            handle,
            loader,
            extent,
            surface_format,
            images,
            image_views,
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
            self.loader.acquire_next_image(
                self.handle,
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
            Ok((_, true)) => {
                // Image acquired but the swapchain is suboptimal
                log::warn!("Swapchain is suboptimal! Request a rebuild...");
                Ok(AcquireImageStatus::NeedsRebuild)
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                Ok(AcquireImageStatus::NeedsRebuild)
            }
            Err(error) => Err(error).with_context(trace!(
                "Error while acquiring the swapchain image!"
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
        let (handle, extent, surface_format) = create::create_swapchain(
            rc,
            &self.loader,
            framebuffer_size,
            Some(self.handle),
        )
        .with_context(trace!("Unable to rebuild the swapchain!"))?;

        unsafe { self.destroy(rc) };

        self.handle = handle;
        self.extent = extent;
        self.surface_format = surface_format;

        self.images = unsafe {
            self.loader
                .get_swapchain_images(handle)
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

    /// Destroy the swapchain.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The swapchain must not be in use by the GPU when it is destroyed.
    /// - Destroy must only be called on a single swapchain instance, even if
    ///   there are multiple clones.
    /// - The swapchain must not be used after calling destroy.
    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        image_views::destroy_image_views(rc, &self.image_views);
        self.image_views.clear();
        self.images.clear();

        self.loader.destroy_swapchain(self.handle, None);
        self.handle = vk::SwapchainKHR::null();
        self.extent = vk::Extent2D {
            width: 0,
            height: 0,
        };
    }
}
