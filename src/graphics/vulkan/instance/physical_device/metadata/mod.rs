mod support;

use {anyhow::Result, ash::vk};

/// Represents the PhysicalDevice features and properties for a particular
/// device.
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceMetadata {
    pub queue_family_properties: Vec<vk::QueueFamilyProperties>,
    pub supported_extensions: Vec<String>,
    pub physical_device_features: vk::PhysicalDeviceFeatures,
    pub physical_device_vulkan_13_features: vk::PhysicalDeviceVulkan13Features,
    pub descriptor_indexing_features:
        vk::PhysicalDeviceDescriptorIndexingFeatures,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
}

/// # Safety:
///
/// Safe because:
/// - the *mut c_void pointers inside the Vulkan feature structs are not
///   directly used while working with PhysicalDeviceMetadata.
unsafe impl Send for PhysicalDeviceMetadata {}

impl PhysicalDeviceMetadata {
    /// Query Vulkan for all metadata for a given physical device.
    pub fn for_physical_device(
        ash: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> Result<Self> {
        let (
            physical_device_features,
            physical_device_vulkan_13_features,
            descriptor_indexing_features,
        ) = get_physical_device_features(ash, physical_device);
        Ok(Self {
            queue_family_properties: get_queue_family_properties(
                ash,
                physical_device,
            ),
            supported_extensions: get_supported_physical_device_extensions(
                ash,
                physical_device,
            )?,
            physical_device_properties: get_physical_device_properties(
                ash,
                physical_device,
            ),
            physical_device_features,
            physical_device_vulkan_13_features,
            descriptor_indexing_features,
        })
    }

    /// Returns a readable copy of the Device Name.
    pub fn device_name(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(
                self.physical_device_properties
                    .device_name
                    .iter()
                    .filter(|&c| *c != 0)
                    .map(|&c| c as u8)
                    .collect(),
            )
        }
    }

    /// Returns true when all of the requested features are supported.
    pub fn supports_features(
        &self,
        requested_features: vk::PhysicalDeviceFeatures,
    ) -> bool {
        support::are_physical_device_features_supported(
            &requested_features,
            &self.physical_device_features,
        )
    }

    /// Returns true when all of the requested features are supported.
    pub fn supports_vulkan_13_features(
        &self,
        requested_features: vk::PhysicalDeviceVulkan13Features,
    ) -> bool {
        support::are_physical_device_vulkan_13_features_supported(
            &requested_features,
            &self.physical_device_vulkan_13_features,
        )
    }

    /// Returns true when all of the requested features are supported.
    pub fn supports_descriptor_indexing_features(
        &self,
        requested_features: vk::PhysicalDeviceDescriptorIndexingFeatures,
    ) -> bool {
        support::are_descriptor_indexing_features(
            &requested_features,
            &self.descriptor_indexing_features,
        )
    }
}

/// Get the physical device properties for a device.
fn get_physical_device_properties(
    ash: &ash::Instance,
    physical_device: &vk::PhysicalDevice,
) -> vk::PhysicalDeviceProperties {
    let mut properties = vk::PhysicalDeviceProperties2::default();
    unsafe {
        ash.get_physical_device_properties2(*physical_device, &mut properties);
    }
    properties.properties
}

/// Get the features for a device
fn get_physical_device_features(
    ash: &ash::Instance,
    physical_device: &vk::PhysicalDevice,
) -> (
    vk::PhysicalDeviceFeatures,
    vk::PhysicalDeviceVulkan13Features,
    vk::PhysicalDeviceDescriptorIndexingFeatures,
) {
    let mut physical_device_vulkan_13_features =
        vk::PhysicalDeviceVulkan13Features::default();
    let mut descriptor_indexing_features =
        vk::PhysicalDeviceDescriptorIndexingFeatures::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default();

    unsafe {
        descriptor_indexing_features.p_next =
            &mut physical_device_vulkan_13_features as *mut _
                as *mut std::ffi::c_void;
        features2.p_next = &mut descriptor_indexing_features as *mut _
            as *mut std::ffi::c_void;

        ash.get_physical_device_features2(*physical_device, &mut features2);

        descriptor_indexing_features.p_next = std::ptr::null_mut();
        features2.p_next = std::ptr::null_mut();
    }

    (
        features2.features,
        physical_device_vulkan_13_features,
        descriptor_indexing_features,
    )
}

fn get_supported_physical_device_extensions(
    ash: &ash::Instance,
    physical_device: &vk::PhysicalDevice,
) -> Result<Vec<String>> {
    let extension_properties =
        unsafe { ash.enumerate_device_extension_properties(*physical_device)? };
    let names = extension_properties
        .iter()
        .filter_map(|properties| {
            String::from_utf8(
                properties
                    .extension_name
                    .iter()
                    .filter(|&c| *c != 0)
                    .map(|c| *c as u8)
                    .collect::<Vec<u8>>(),
            )
            .ok()
        })
        .collect();
    Ok(names)
}

fn get_queue_family_properties(
    ash: &ash::Instance,
    physical_device: &vk::PhysicalDevice,
) -> Vec<vk::QueueFamilyProperties> {
    unsafe { ash.get_physical_device_queue_family_properties(*physical_device) }
}
