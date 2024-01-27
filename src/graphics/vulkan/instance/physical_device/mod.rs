mod metadata;

pub use metadata::PhysicalDeviceMetadata;
use {crate::graphics::vulkan::instance::Instance, anyhow::Result, ash::vk};

impl Instance {
    /// Enumerate all of the physical devices on the system.
    ///
    /// Physical Devices are filtered based on their supported features to
    /// select only the ones which support the operations required by this
    /// application.
    pub fn enumerate_devices_with_required_features(
        &self,
    ) -> Result<Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)>> {
        let physical_devices =
            unsafe { self.ash.enumerate_physical_devices()? };

        let metadata: Vec<(vk::PhysicalDevice, PhysicalDeviceMetadata)> =
            physical_devices
                .iter()
                .filter_map(|&device| {
                    PhysicalDeviceMetadata::for_physical_device(
                        &self.ash, &device,
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
}
