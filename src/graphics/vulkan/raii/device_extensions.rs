use {crate::graphics::vulkan::raii, anyhow::Result, ash::vk, std::sync::Arc};

macro_rules! device_extension {
    ($name: ident, $arc_name: ident, $ext_type: ty, $raw_type: ty, $destroy: ident) => {
        /// RAII wrapper for $name
        pub struct $name {
            pub ext: $ext_type,
            pub raw: $raw_type,
            pub device: Arc<raii::Device>,
        }
        pub type $arc_name = Arc<$name>;

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ext", &"<extension loader>")
                    .field("raw", &self.raw)
                    .field("device", &self.device)
                    .finish()
            }
        }

        impl std::ops::Deref for $name {
            type Target = $raw_type;

            fn deref(&self) -> &Self::Target {
                &self.raw
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { self.ext.$destroy(self.raw, None) }
            }
        }
    };
}

device_extension!(
    Swapchain,
    SwapchainArc,
    ash::extensions::khr::Swapchain,
    vk::SwapchainKHR,
    destroy_swapchain
);

impl Swapchain {
    pub fn new(
        device: Arc<raii::Device>,
        create_info: &vk::SwapchainCreateInfoKHR,
    ) -> Result<Arc<Self>> {
        let ext =
            ash::extensions::khr::Swapchain::new(&device.instance, &device);
        let raw = unsafe { ext.create_swapchain(&create_info, None)? };
        Ok(Arc::new(Self { ext, raw, device }))
    }
}
