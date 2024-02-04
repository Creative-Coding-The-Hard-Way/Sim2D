use {
    crate::{
        graphics::vulkan::{
            memory::{Allocator, Block},
            raii,
        },
        trace,
    },
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

impl Allocator for DeviceAllocator {
    fn allocate(
        &mut self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<Block> {
        let device_memory = self.allocate_device_memory(
            memory_requirements,
            property_flags,
            memory_allocate_flags,
        )?;
        let mapped_ptr: *mut std::ffi::c_void =
            if property_flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
                unsafe {
                    self.device
                        .map_memory(
                            device_memory.raw,
                            0,
                            memory_requirements.size,
                            vk::MemoryMapFlags::empty(),
                        )
                        .with_context(trace!("Unable to map device memory!"))?
                }
            } else {
                std::ptr::null_mut()
            };
        Ok(Block {
            mapped_ptr,
            memory: device_memory,
        })
    }

    fn free(&mut self, block: &Block) {
        // no-op! When the block is dropped, the backing memory is freed
    }
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
    pub fn allocate_device_memory(
        &self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<raii::DeviceMemoryArc> {
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
        let raw = unsafe {
            self.device
                .allocate_memory(&allocate_info, None)
                .with_context(trace!("Unable to allocate memory!"))?
        };
        Ok(raii::DeviceMemory::new(self.device.clone(), raw))
    }
}
