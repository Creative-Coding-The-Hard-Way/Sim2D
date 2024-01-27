mod surface;

pub use surface::Surface;
use {
    crate::graphics::vulkan::instance::{
        physical_device::PhysicalDeviceMetadata, Instance,
    },
    anyhow::{Context, Result},
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
            pick_physical_device(&instance)?;
        log::info!(
            "Chose physical device: {}",
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

fn pick_physical_device(
    instance: &Instance,
) -> Result<(vk::PhysicalDevice, PhysicalDeviceMetadata)> {
    let useable_devices: Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)> =
        instance
            .enumerate_devices_with_required_features()?
            .into_iter()
            .filter(|(_, metadata)| {
                let swapchain_extension_name =
                    ash::extensions::khr::Swapchain::name()
                        .to_owned()
                        .into_string()
                        .unwrap();
                let has_extensions = metadata
                    .supported_extensions
                    .contains(&swapchain_extension_name);
                log::trace!(
                    "{} has required extensions? {}",
                    metadata.device_name(),
                    has_extensions
                );
                has_extensions
            })
            .collect();

    let find_device = |device_type: vk::PhysicalDeviceType| -> Option<(
        vk::PhysicalDevice,
        PhysicalDeviceMetadata,
    )> {
        useable_devices
            .iter()
            .find(|(_device, metadata)| {
                metadata.physical_device_properties.device_type == device_type
            })
            .cloned()
    };

    if let Some(entry) = find_device(vk::PhysicalDeviceType::DISCRETE_GPU) {
        return Ok(entry);
    }

    if let Some(entry) = find_device(vk::PhysicalDeviceType::INTEGRATED_GPU) {
        return Ok(entry);
    }

    useable_devices
        .first()
        .cloned()
        .context("Unable to find a suitable physical device!")
}
