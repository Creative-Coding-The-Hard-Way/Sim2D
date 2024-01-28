mod logical_device;
mod pick_physical_device;
mod queue_families;
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
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
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
        let queue_families = queue_families::QueueFamilies::new(
            physical_device,
            &physical_device_metadata,
            &surface,
        )?;
        let device = logical_device::create_logical_device(
            &instance,
            physical_device,
            &queue_families,
        )?;
        let (graphics_queue, present_queue) =
            queue_families.get_queues_from_device(&device);
        Ok(Self {
            instance,
            surface,
            physical_device,
            physical_device_metadata,
            device,
            graphics_queue,
            present_queue,
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
        self.device.destroy_device(None);
        self.surface.destroy();
        self.instance.destroy();
    }
}
