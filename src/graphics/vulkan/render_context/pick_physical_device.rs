use {
    crate::graphics::vulkan::instance::{
        physical_device::PhysicalDeviceMetadata, Instance,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Pick a suitable physical device for this application.
///
/// A suitable physical device is one that has the required features,
/// extensions, and queues.
pub(super) fn find_suitable_device(
    instance: &Instance,
) -> Result<(vk::PhysicalDevice, PhysicalDeviceMetadata)> {
    let useable_devices: Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)> =
        instance
            .enumerate_devices_with_required_features()?
            .into_iter()
            .filter(|(_, metadata)| {
                let has_graphics =
                    metadata.queue_family_properties.iter().any(|properties| {
                        properties
                            .queue_flags
                            .contains(vk::QueueFlags::GRAPHICS)
                    });
                has_graphics
            })
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
