use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
};

/// A memory allocator which directly allocates memory from the logical device.
#[derive(Debug, Clone)]
pub struct DeviceAllocator {
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub physical_device: vk::PhysicalDevice,
    pub device: raii::DeviceArc,
}

impl DeviceAllocator {
    /// Create a new allocator instance.
    pub fn new(
        device: raii::DeviceArc,
        physical_device: vk::PhysicalDevice,
    ) -> Self {
        let memory_properties = unsafe {
            device
                .instance
                .get_physical_device_memory_properties(physical_device)
        };
        Self {
            memory_properties,
            physical_device,
            device,
        }
    }

    /// Allocate memory based on the provided requirements and properties.
    ///
    /// The caller must free the memory from the device before it is destroyed.
    pub fn allocate_memory(
        &self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<vk::DeviceMemory> {
        let memory_type_index = self
            .memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, &memory_type)| {
                let type_bits = 1 << *index;
                let is_required_type =
                    type_bits & memory_requirements.memory_type_bits != 0;
                let has_required_properties =
                    memory_type.property_flags.contains(property_flags);
                is_required_type && has_required_properties
            })
            .map(|(index, _memory_type)| index)
            .with_context(trace!("Unable to get suitable memory type"))?;
        let allocate_flags_info = vk::MemoryAllocateFlagsInfo {
            flags: memory_allocate_flags,
            ..Default::default()
        };
        let allocate_info = vk::MemoryAllocateInfo {
            p_next: &allocate_flags_info as *const _ as *const _,
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type_index as u32,
            ..Default::default()
        };
        unsafe {
            self.device
                .allocate_memory(&allocate_info, None)
                .with_context(trace!("Unable to allocate memory!"))
        }
    }

    /// Free previously-allocated Vulkan device memory.
    pub fn free_memory(&self, memory: vk::DeviceMemory) {
        unsafe { self.device.free_memory(memory, None) }
    }
}
