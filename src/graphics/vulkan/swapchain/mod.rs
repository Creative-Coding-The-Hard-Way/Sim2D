mod create;

use {super::render_context::RenderContext, anyhow::Result, ash::vk};

/// The application swapchain.
#[derive(Clone)]
pub struct Swapchain {
    pub handle: vk::SwapchainKHR,
    pub loader: ash::extensions::khr::Swapchain,
    pub extent: vk::Extent2D,
}

impl Swapchain {
    pub fn new(
        rc: &RenderContext,
        framebuffer_size: (u32, u32),
    ) -> Result<Self> {
        let loader =
            ash::extensions::khr::Swapchain::new(&rc.instance.ash, &rc.device);
        let (handle, extent) =
            create::create_swapchain(rc, &loader, framebuffer_size, None)?;
        Ok(Self {
            handle,
            loader,
            extent,
        })
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
    pub unsafe fn destroy(&mut self) {
        self.loader.destroy_swapchain(self.handle, None);
        self.handle = vk::SwapchainKHR::null();
        self.extent = vk::Extent2D {
            width: 0,
            height: 0,
        };
    }
}
