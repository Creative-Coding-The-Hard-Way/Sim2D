use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

macro_rules! instance_extension {
    (
        $name: ident,
        $arc_name: ident,
        $ext_type: ty,
        $raw_type: ty,
        $destroy: ident
    ) => {
        /// RAII wrapper for $name
        pub struct $name {
            pub ext: $ext_type,
            pub raw: $raw_type,
            pub instance: Arc<raii::Instance>,
        }

        pub type $arc_name = Arc<$name>;

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ext", &"<extension loader>")
                    .field("raw", &self.raw)
                    .field("instance", &self.instance)
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

instance_extension!(
    DebugUtils,
    DebugUtilsArc,
    ash::extensions::ext::DebugUtils,
    vk::DebugUtilsMessengerEXT,
    destroy_debug_utils_messenger
);

impl DebugUtils {
    pub fn new(
        instance: Arc<raii::Instance>,
        create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
    ) -> Result<Arc<Self>> {
        let ext = ash::extensions::ext::DebugUtils::new(
            &instance.entry,
            &instance.raw,
        );
        let raw =
            unsafe { ext.create_debug_utils_messenger(&create_info, None)? };
        Ok(Arc::new(Self { ext, raw, instance }))
    }
}

instance_extension!(
    Surface,
    SurfaceArc,
    ash::extensions::khr::Surface,
    vk::SurfaceKHR,
    destroy_surface
);

impl Surface {
    pub fn new(
        instance: Arc<raii::Instance>,
        raw: vk::SurfaceKHR,
    ) -> Result<Arc<Self>> {
        let ext =
            ash::extensions::khr::Surface::new(&instance.entry, &instance);
        Ok(Arc::new(Self { raw, ext, instance }))
    }

    pub fn from_glfw_window(
        instance: Arc<raii::Instance>,
        window: &glfw::Window,
    ) -> Result<Arc<Self>> {
        let handle = {
            let mut surface = ash::vk::SurfaceKHR::null();
            window
                .create_window_surface(
                    instance.raw.handle(),
                    std::ptr::null(),
                    &mut surface,
                )
                .result()
                .with_context(trace!(
                    "Unable to create the Vulkan SurfaceKHR with GLFW!"
                ))?;
            surface
        };
        Ok(raii::Surface::new(instance, handle)?)
    }
}
