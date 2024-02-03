mod create_instance;
mod debug_logging;

use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::extensions::ext::DebugUtils,
};

/// The Vulkan library instance and associated information.
#[derive(Clone)]
pub struct Instance {
    pub ash: raii::InstanceArc,
    extensions: Vec<String>,
    _debug_utils: Option<raii::DebugUtilsArc>,
}

impl Instance {
    /// Create a new Vulkan instance.
    pub fn new<S>(app_name: S, required_extensions: &[String]) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let extensions = {
            let mut extensions = required_extensions.to_vec();
            if cfg!(debug_assertions) {
                extensions
                    .push(DebugUtils::name().to_str().unwrap().to_owned());
            }
            extensions
        };
        let ash = unsafe {
            create_instance::create_instance(app_name, &extensions)
                .with_context(trace!("Unable to create the Vulkan instance!"))?
        };
        let debug_utils = debug_logging::setup_debug_logging(ash.clone())
            .with_context(trace!("Unable to setup debug logging!"))?;
        Ok(Self {
            ash,
            extensions,
            _debug_utils: debug_utils,
        })
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("ash", &"<Ash Library Instance>")
            .field("extensions", &self.extensions)
            .finish()
    }
}
