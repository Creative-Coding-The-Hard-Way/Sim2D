mod pick_physical_device;
mod surface;

pub use surface::Surface;
use {
    crate::graphics::vulkan::instance::{
        physical_device::PhysicalDeviceMetadata, Instance,
    },
    anyhow::Result,
    ash::vk,
};

/// The Vulkan rendering context.
///
/// The context contains all of the core resources required by most Vulkan
/// graphics applications. This includes the logical device, the physical
/// device, the surface, and the relevant queue handles.
#[derive(Clone)]
pub struct RenderContext {
    pub instance: Instance,
    pub surface: Surface,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_metadata: PhysicalDeviceMetadata,
}

impl RenderContext {
    /// Create a new RenderContext for this application.
    pub fn new(instance: Instance, surface: Surface) -> Result<Self> {
        let (physical_device, physical_device_metadata) =
            pick_physical_device::find_suitable_device(&instance)?;
        log::info!(
            "Chosen physical device: {}",
            physical_device_metadata.device_name()
        );
        Ok(Self {
            instance,
            surface,
            physical_device,
            physical_device_metadata,
        })
    }

    /// Destroy the context.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - This method must be called only once, regardless of how many clones
    ///   exist. (e.g. if there are 3 clones, destroy should be called 1 time
    ///   with only one of the clones).
    /// - All GPU resources created with the instance and logical device must be
    ///   destroyed before calling this method.
    pub unsafe fn destroy(&mut self) {
        self.surface.destroy();
        self.instance.destroy();
    }
}
