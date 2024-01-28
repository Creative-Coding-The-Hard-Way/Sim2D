mod create_instance;
mod debug_logging;

use {
    anyhow::Result,
    ash::{extensions::ext::DebugUtils, vk},
};

#[derive(Clone)]
pub struct Instance {
    pub entry: ash::Entry,
    pub ash: ash::Instance,
    extensions: Vec<String>,
    debug_utils: Option<(DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

impl Instance {
    /// Create a new Vulkan instance.
    pub fn new<S>(app_name: S, required_extensions: &[String]) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let entry = unsafe { ash::Entry::load()? };
        let extensions = {
            let mut extensions = required_extensions.to_vec();
            if cfg!(debug_assertions) {
                extensions
                    .push(DebugUtils::name().to_str().unwrap().to_owned());
            }
            extensions
        };
        let ash = unsafe {
            create_instance::create_instance(&entry, app_name, &extensions)?
        };
        let debug_utils = debug_logging::setup_debug_logging(&entry, &ash)?;
        Ok(Self {
            entry,
            ash,
            extensions,
            debug_utils,
        })
    }

    /// Destroy the Vulkan instance.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - Any and all resources created with the instance must be destroyed
    ///   before this method is called.
    /// - The Instance should only be destroyed once. (e.g. if there are many
    ///   clone()s, only one should call destroy().)
    /// - It is invalid to use the instance after destroy() has been called.
    pub unsafe fn destroy(&mut self) {
        if let Some((debug_utils, messenger)) = self.debug_utils.take() {
            debug_utils.destroy_debug_utils_messenger(messenger, None);
        }
        self.ash.destroy_instance(None);
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("entry", &"<Vulkan Library Entrypoint>")
            .field("ash", &"<Ash Library Instance>")
            .field("extensions", &self.extensions)
            .finish()
    }
}
