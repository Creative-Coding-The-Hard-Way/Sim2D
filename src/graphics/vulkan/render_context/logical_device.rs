use {
    crate::{
        graphics::vulkan::{
            raii, render_context::queue_families::QueueFamilies,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

pub(super) fn create_logical_device(
    instance: raii::InstanceArc,
    physical_device: vk::PhysicalDevice,
    queue_families: &QueueFamilies,
) -> Result<raii::DeviceArc> {
    // Pick Queues
    let queue_create_infos = queue_families.get_queue_create_info();

    // Set device extensions
    let extension_name = ash::extensions::khr::Swapchain::name();
    let extensions = [extension_name.as_ptr()];

    // Pick Device Features
    // ALSO UPDATE: physical_device::enumerate_devices_with_required_features
    let mut physical_device_buffer_device_address_features =
        vk::PhysicalDeviceBufferDeviceAddressFeatures {
            buffer_device_address: vk::TRUE,
            ..Default::default()
        };
    let mut physical_device_vulkan_13_features =
        vk::PhysicalDeviceVulkan13Features {
            ..Default::default()
        };
    let mut descriptor_indexing_features =
        vk::PhysicalDeviceDescriptorIndexingFeatures {
            ..Default::default()
        };
    let mut features2 = vk::PhysicalDeviceFeatures2 {
        features: vk::PhysicalDeviceFeatures {
            ..Default::default()
        },
        ..Default::default()
    };
    features2.p_next = &mut descriptor_indexing_features as *mut _ as *mut _;
    descriptor_indexing_features.p_next =
        &mut physical_device_vulkan_13_features as *mut _ as *mut _;
    physical_device_vulkan_13_features.p_next =
        &mut physical_device_buffer_device_address_features as *mut _ as *mut _;

    // Create the device
    let create_info = vk::DeviceCreateInfo {
        p_next: &mut features2 as *mut _ as *mut std::ffi::c_void,
        queue_create_info_count: queue_create_infos.len() as u32,
        p_queue_create_infos: queue_create_infos.as_ptr(),
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),

        // Null because p_next contains the physical device features
        p_enabled_features: std::ptr::null(),
        ..Default::default()
    };
    Ok(raii::Device::new(instance, physical_device, &create_info)
        .with_context(trace!("Unable to create the Logical Device"))?)
}
