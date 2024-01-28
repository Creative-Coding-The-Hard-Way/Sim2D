use {
    crate::graphics::vulkan::{
        instance::physical_device::PhysicalDeviceMetadata,
        render_context::Surface,
    },
    anyhow::{Context, Result},
    ash::vk,
};

static PRIORITIES: [f32; 16] = [1.0; 16];

/// Represents all of the Queue Families required by the application.
#[derive(Debug, Copy, Clone)]
pub(super) struct QueueFamilies {
    pub graphics_family_index: u32,
    pub present_family_index: u32,
}

impl QueueFamilies {
    /// Select queue families based on the given physical device metadata and
    /// the presentation surface.
    pub fn new(
        physical_device: vk::PhysicalDevice,
        metadata: &PhysicalDeviceMetadata,
        surface: &Surface,
    ) -> Result<Self> {
        let (graphics_family_index, _) = metadata
            .queue_family_properties
            .iter()
            .enumerate()
            .find(|(_, &properties)| {
                properties.queue_flags.contains(
                    vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE,
                )
            })
            .context("Unable to find a Queue Family that supports GRAPHICS")?;
        let (present_family_index, _) = {
            let non_graphics_present_queue = metadata
                .queue_family_properties
                .iter()
                .enumerate()
                .filter(|(queue_family_index, _)| {
                    *queue_family_index != graphics_family_index
                })
                .find(|(queue_family_index, _)| unsafe {
                    surface
                        .loader
                        .get_physical_device_surface_support(
                            physical_device,
                            *queue_family_index as u32,
                            surface.handle,
                        )
                        .unwrap_or(false)
                });
            let maybe_graphics_present_queue =
                metadata.queue_family_properties.iter().enumerate().find(
                    |(queue_family_index, _)| unsafe {
                        surface
                            .loader
                            .get_physical_device_surface_support(
                                physical_device,
                                *queue_family_index as u32,
                                surface.handle,
                            )
                            .unwrap_or(false)
                    },
                );
            non_graphics_present_queue
                .or(maybe_graphics_present_queue)
                .context("Unable to select a present queue family!")?
        };
        Ok(Self {
            graphics_family_index: graphics_family_index as u32,
            present_family_index: present_family_index as u32,
        })
    }

    /// Get the queue create info structs for the selected queue families.
    pub(super) fn get_queue_create_info(
        &self,
    ) -> Vec<vk::DeviceQueueCreateInfo> {
        let mut create_infos = vec![];
        create_infos.push(vk::DeviceQueueCreateInfo {
            queue_family_index: self.graphics_family_index,
            queue_count: 1,
            p_queue_priorities: PRIORITIES.as_ptr(),
            ..Default::default()
        });
        if self.present_family_index != self.graphics_family_index {
            create_infos.push(vk::DeviceQueueCreateInfo {
                queue_family_index: self.present_family_index,
                queue_count: 1,
                p_queue_priorities: PRIORITIES.as_ptr(),
                ..Default::default()
            });
        }
        create_infos
    }

    /// Get the required queues from the device.
    ///
    /// # Returns
    ///
    /// A tuple of
    /// 1. A queue which supports GRAPHICS and COMPUTE operations
    /// 2. A queue which supports presenting to the SurfaceKHR
    ///
    /// Note that the queue handles can be the same depending on the system.
    pub(super) fn get_queues_from_device(
        &self,
        device: &ash::Device,
    ) -> (vk::Queue, vk::Queue) {
        let graphics_queue =
            unsafe { device.get_device_queue(self.graphics_family_index, 0) };
        let present_queue =
            unsafe { device.get_device_queue(self.present_family_index, 0) };
        (graphics_queue, present_queue)
    }
}
