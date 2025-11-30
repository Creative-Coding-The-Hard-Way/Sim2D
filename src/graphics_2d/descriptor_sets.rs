use {
    super::{UniformData, Vertex},
    crate::Gfx,
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::{CPUBuffer, UniformBuffer, raii},
};

pub fn create_descriptor_set_layout(
    gfx: &Gfx,
) -> Result<raii::DescriptorSetLayout> {
    let bindings = [
        vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: std::ptr::null(),
            ..Default::default()
        },
        vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: std::ptr::null(),
            ..Default::default()
        },
    ];
    raii::DescriptorSetLayout::new(
        "FirstTriangleDescLayout",
        gfx.vulkan.device.clone(),
        &vk::DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        },
    )
}

pub fn create_descriptor_pool(gfx: &Gfx) -> Result<raii::DescriptorPool> {
    let frame_count = gfx.frames_in_flight.frame_count() as u32;
    let pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: frame_count,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: frame_count,
        },
    ];
    raii::DescriptorPool::new(
        "FirstTriangleDescriptorPool",
        gfx.vulkan.device.clone(),
        &vk::DescriptorPoolCreateInfo {
            max_sets: frame_count,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_ptr(),
            ..Default::default()
        },
    )
}

/// allocates one descriptor set for each frame in flight
pub fn allocate_descriptor_sets(
    gfx: &Gfx,
    pool: &raii::DescriptorPool,
    layout: &raii::DescriptorSetLayout,
) -> Result<Vec<vk::DescriptorSet>> {
    unsafe {
        let layouts = (0..gfx.frames_in_flight.frame_count())
            .map(|_| layout.raw)
            .collect::<Vec<_>>();
        gfx.vulkan
            .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                descriptor_pool: pool.raw,
                descriptor_set_count: layouts.len() as u32,
                p_set_layouts: layouts.as_ptr(),
                ..Default::default()
            })
            .context("Error while allocating descriptor sets")
    }
}

pub fn write_descriptor_sets(
    gfx: &Gfx,
    descriptor_sets: &[vk::DescriptorSet],
    uniform_buffer: &UniformBuffer<UniformData>,
    vertex_buffers: &[CPUBuffer<Vertex>],
) {
    let vertex_buffer_infos: Vec<vk::DescriptorBufferInfo> = descriptor_sets
        .iter()
        .zip(vertex_buffers.iter())
        .map(|(_, vertex_buffer)| vk::DescriptorBufferInfo {
            buffer: vertex_buffer.buffer(),
            offset: 0,
            range: vertex_buffer.size_in_bytes(),
        })
        .collect();
    let uniform_buffer_infos: Vec<vk::DescriptorBufferInfo> = descriptor_sets
        .iter()
        .enumerate()
        .map(|(index, _descriptor_set)| vk::DescriptorBufferInfo {
            buffer: uniform_buffer.buffer(),
            offset: uniform_buffer.offset_for_index(index),
            range: std::mem::size_of::<UniformData>() as u64,
        })
        .collect();
    let writes: Vec<vk::WriteDescriptorSet> = descriptor_sets
        .iter()
        .enumerate()
        .flat_map(|(index, descriptor_set)| {
            [
                vk::WriteDescriptorSet {
                    dst_set: *descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: &uniform_buffer_infos[index],
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                },
                vk::WriteDescriptorSet {
                    dst_set: *descriptor_set,
                    dst_binding: 1,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: &vertex_buffer_infos[index],
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                },
            ]
        })
        .collect();
    unsafe { gfx.vulkan.update_descriptor_sets(&writes, &[]) };
}
