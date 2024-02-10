use {
    crate::graphics::vulkan::{memory, raii, render_context::RenderContext},
    anyhow::Result,
    ash::vk,
};

struct TransformUniformBuffer {
    transform: [[f32; 4]; 4],
}

pub struct Transform {
    pub buffer: raii::BufferArc,
    pub block: memory::OwnedBlock,
    pub descriptor_pool: raii::DescriptorPoolArc,
    pub descriptor_set: vk::DescriptorSet,
}

impl Transform {
    pub fn new(rc: &RenderContext) -> Result<Self> {
        let buffer = {
            let create_info = vk::BufferCreateInfo {
                size: std::mem::size_of::<TransformUniformBuffer>() as u64,
                usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &rc.graphics_queue_index,
                ..Default::default()
            };
            raii::Buffer::new(rc.device.clone(), &create_info)?
        };
        let block = {
            let memory_requirements =
                unsafe { rc.device.get_buffer_memory_requirements(buffer.raw) };
            rc.allocator.allocate(
                memory_requirements,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
                vk::MemoryAllocateFlags::empty(),
            )?
        };
        unsafe {
            rc.device
                .bind_buffer_memory(buffer.raw, block.memory.raw, 0)?;
        };
        let descriptor_pool = {
            let pool_size = vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
            };
            let create_info = vk::DescriptorPoolCreateInfo {
                max_sets: 1,
                pool_size_count: 1,
                p_pool_sizes: &pool_size,
                ..Default::default()
            };
            raii::DescriptorPool::new(rc.device.clone(), &create_info)?
        };
        let descriptor_set = {
            let allocate_info = vk::DescriptorSetAllocateInfo {
                descriptor_pool: descriptor_pool.raw,
                descriptor_set_count: 1,
                p_set_layouts: todo!(),
                ..Default::default()
            };
            unsafe { rc.device.allocate_descriptor_sets(&allocate_info)?[0] }
        };
        Ok(Self {
            buffer,
            block,
            descriptor_pool,
            descriptor_set,
        })
    }
}
