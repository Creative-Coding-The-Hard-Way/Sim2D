mod window_state;

use {
    crate::graphics::vulkan_api::RenderDevice,
    anyhow::{bail, Context, Result},
    ash::{vk, vk::Handle},
    ccthw_ash_instance::{PhysicalDeviceFeatures, VulkanInstance},
    glfw::{ClientApiHint, WindowEvent, WindowHint, WindowMode},
    std::sync::{mpsc::Receiver, Arc},
};

/// All resources required for running a single-windowed GLFW application which
/// renders graphics using Vulkan.
///
/// GlfwWindow derefs as a raw GLFW window handle so application state can
/// configure the window however is convenient.
pub struct GlfwWindow {
    window_handle: glfw::Window,

    /// The GLFW library instance.
    glfw: glfw::Glfw,
}

impl GlfwWindow {
    /// Create a new GLFW window.
    ///
    /// The window starts in "windowed" mode and can be toggled into fullscreen
    /// or resized by the application.
    ///
    /// # Params
    ///
    /// * `window_title` - The title shown on the window's top bar.
    pub fn new(
        window_title: impl AsRef<str>,
    ) -> Result<(Self, Receiver<(f64, WindowEvent)>)> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        if !glfw.vulkan_supported() {
            bail!("Vulkan isn't supported by glfw on this platform!");
        }

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        glfw.window_hint(WindowHint::ScaleToMonitor(true));

        let (mut window_handle, event_receiver) = glfw
            .create_window(
                800,
                600,
                window_title.as_ref(),
                WindowMode::Windowed,
            )
            .context("Creating the GLFW Window failed!")?;

        window_handle.set_all_polling(true);

        Ok((
            Self {
                window_handle,
                glfw,
            },
            event_receiver,
        ))
    }

    /// Create a render device for the application.
    ///
    /// # Safety
    ///
    /// The application is responsible for synchronizing access to all Vulkan
    /// resources and destroying the render device at exit.
    pub unsafe fn create_render_device(&self) -> Result<Arc<RenderDevice>> {
        let mut device_features = PhysicalDeviceFeatures::default();

        // enable synchronization2 for queue_submit2
        device_features.vulkan_13_features_mut().synchronization2 = vk::TRUE;

        // enable descriptor indexing for bindless graphics
        device_features
            .descriptor_indexing_features_mut()
            .shader_sampled_image_array_non_uniform_indexing = vk::TRUE;
        device_features
            .descriptor_indexing_features_mut()
            .runtime_descriptor_array = vk::TRUE;

        let instance = self.create_vulkan_instance()?;

        let surface = {
            let mut surface_handle: u64 = 0;
            let result =
                vk::Result::from_raw(self.window_handle.create_window_surface(
                    instance.ash().handle().as_raw() as usize,
                    std::ptr::null(),
                    &mut surface_handle,
                ) as i32);
            if result != vk::Result::SUCCESS {
                bail!("Unable to create a Vulkan SurfaceKHR with GLFW!");
            }
            vk::SurfaceKHR::from_raw(surface_handle)
        };

        let device = RenderDevice::new(instance, device_features, surface)
            .context("Unable to create the render device!")?;

        log::debug!("{}", device);

        Ok(Arc::new(device))
    }

    /// Create a Vulkan instance with extensions and layers configured to
    /// such that it can present swapchain frames to the window.
    ///
    /// # Safety
    ///
    /// The application is responsible for synchronizing access to all Vulkan
    /// resources and destroying the Vulkan instance at exit.
    unsafe fn create_vulkan_instance(&self) -> Result<VulkanInstance> {
        let instance_extensions: &[String] = &[];
        let instance_layers: &[String] = &[];

        let mut all_instance_extensions =
            self.glfw.get_required_instance_extensions().context(
                "Cannot get the required instance extensions for this platform",
            )?;
        all_instance_extensions.extend_from_slice(instance_extensions);

        let mut all_layers = instance_layers.to_vec();
        if cfg!(debug_assertions) {
            all_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
        }

        unsafe {
            VulkanInstance::new(&all_instance_extensions, &all_layers)
                .context("Error createing the Vulkan instance!")
        }
    }
}

impl std::ops::Deref for GlfwWindow {
    type Target = glfw::Window;

    fn deref(&self) -> &Self::Target {
        &self.window_handle
    }
}

impl std::ops::DerefMut for GlfwWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window_handle
    }
}
