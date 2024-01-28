mod metadata;

pub use metadata::PhysicalDeviceMetadata;
use {
    crate::{
        graphics::vulkan::render_context::{Instance, Surface},
        trace,
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
    surface: &Surface,
) -> Result<(vk::PhysicalDevice, PhysicalDeviceMetadata)> {
    let useable_devices: Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)> =
        enumerate_devices_with_required_features(instance)
            .with_context(trace!(
                "Unable to enumerate devices with required features!"
            ))?
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
            .filter(|(device, _)| {
                let has_surface_formats = unsafe {
                    surface
                        .loader
                        .get_physical_device_surface_formats(
                            *device,
                            surface.handle,
                        )
                        .map(|formats| !formats.is_empty())
                        .unwrap_or(false)
                };
                let has_present_modes = unsafe {
                    surface
                        .loader
                        .get_physical_device_surface_present_modes(
                            *device,
                            surface.handle,
                        )
                        .map(|modes| !modes.is_empty())
                        .unwrap_or(false)
                };
                has_surface_formats && has_present_modes
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

/// Enumerate all of the physical devices on the system.
///
/// Physical Devices are filtered based on their supported features to
/// select only the ones which support the operations required by this
/// application.
fn enumerate_devices_with_required_features(
    instance: &Instance,
) -> Result<Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)>> {
    let physical_devices = unsafe {
        instance
            .ash
            .enumerate_physical_devices()
            .with_context(trace!("Unable to enumerate physical devices!"))?
    };

    let metadata: Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)> =
        physical_devices
            .iter()
            .filter_map(|&device| {
                PhysicalDeviceMetadata::for_physical_device(
                    &instance.ash,
                    &device,
                )
                .ok()
                .map(|meta| (device, meta))
            })
            .filter(|(_, metadata)| {
                metadata.supports_features(vk::PhysicalDeviceFeatures {
                    ..Default::default()
                })
            })
            .filter(|(_, metadata)| {
                metadata.supports_vulkan_13_features(
                    vk::PhysicalDeviceVulkan13Features {
                        ..Default::default()
                    },
                )
            })
            .filter(|(_, metadata)| {
                metadata.supports_descriptor_indexing_features(
                    vk::PhysicalDeviceDescriptorIndexingFeatures {
                        shader_sampled_image_array_non_uniform_indexing:
                            vk::TRUE,
                        runtime_descriptor_array: vk::TRUE,
                        ..Default::default()
                    },
                )
            })
            .collect();

    log::info!(
        "found devices with required features: \n{:#?}",
        metadata
            .iter()
            .map(|(_, metadata)| metadata.device_name())
            .collect::<Vec<_>>()
    );

    Ok(metadata)
}
