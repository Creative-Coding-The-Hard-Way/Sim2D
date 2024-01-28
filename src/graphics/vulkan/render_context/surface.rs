use {
    crate::graphics::vulkan::render_context::Instance, anyhow::Result, ash::vk,
};

/// The Vulkan Surface.
#[derive(Clone)]
pub struct Surface {
    pub handle: vk::SurfaceKHR,
    pub loader: ash::extensions::khr::Surface,
}

impl Surface {
    /// Create a new surface from a GLFW Window.
    pub fn from_glfw_window(
        window: &glfw::Window,
        instance: &Instance,
    ) -> Result<Self> {
        let handle = {
            let mut surface = ash::vk::SurfaceKHR::null();
            let result = window.create_window_surface(
                instance.ash.handle(),
                std::ptr::null(),
                &mut surface,
            );
            if result != ash::vk::Result::SUCCESS {
                anyhow::bail!(
                    "Unable to create the Vulkan SurfaceKHR with GLFW! {:?}",
                    result
                );
            }
            surface
        };
        let surface_loader =
            ash::extensions::khr::Surface::new(&instance.entry, &instance.ash);
        Ok(Self {
            handle,
            loader: surface_loader,
        })
    }

    /// Destroy the Surface.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - This method must be called only once, regardless of how many clones
    ///   exist. (e.g. if there are 3 clones, destroy should be called 1 time
    ///   with only one of the clones).
    /// - The Vulkan surface must not be in use by the GPU when this method is
    ///   called.
    pub unsafe fn destroy(&mut self) {
        self.loader.destroy_surface(self.handle, None);
        self.handle = vk::SurfaceKHR::null();
    }
}
