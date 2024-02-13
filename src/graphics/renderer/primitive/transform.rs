use {
    crate::{
        graphics::vulkan::{
            memory, raii,
            render_context::RenderContext,
            sync::{AsyncNBuffer, AsyncNBufferClient},
        },
        math::Mat4f,
    },
    anyhow::Result,
    ash::vk,
};

/// The actual data that is saved in the GPU memory and sent to the shader.
struct TransformUniformBuffer {
    transform: [f32; 16],
}

/// All of the resources required to bind the TransformUniformBuffer and use it
/// in a frame.
pub struct Transform {
    pub buffer: raii::BufferArc,
    pub block: memory::OwnedBlock,
    pub descriptor_pool: raii::DescriptorPoolArc,
    pub descriptor_set: vk::DescriptorSet,
    pub descriptor_set_layout: raii::DescriptorSetLayoutArc,
}

impl Transform {
    /// Create N instances which can be passed back and forth between the
    /// renderer and the client Api.
    pub fn create_n_buffered(
        rc: &RenderContext,
        descriptor_set_layout: raii::DescriptorSetLayoutArc,
        initial_value: &Mat4f,
        count: usize,
    ) -> Result<(AsyncNBuffer<Self>, AsyncNBufferClient<Self>)> {
        let mut transforms = Vec::with_capacity(count);
        for _ in 0..count {
            transforms.push(Self::new(
                rc,
                descriptor_set_layout.clone(),
                initial_value,
            )?);
        }
        AsyncNBuffer::new(transforms)
    }

    /// Create a single new Transform instance.
    fn new(
        rc: &RenderContext,
        descriptor_set_layout: raii::DescriptorSetLayoutArc,
        initial_value: &Mat4f,
    ) -> Result<Self> {
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
            let t = block.mapped_ptr as *mut TransformUniformBuffer;
            (*t).transform.copy_from_slice(initial_value.as_slice());
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
                p_set_layouts: &descriptor_set_layout.raw,
                ..Default::default()
            };
            unsafe { rc.device.allocate_descriptor_sets(&allocate_info)?[0] }
        };
        {
            let buffer_info = vk::DescriptorBufferInfo {
                buffer: buffer.raw,
                offset: 0,
                range: block.size_in_bytes,
            };
            unsafe {
                rc.device.update_descriptor_sets(
                    &[vk::WriteDescriptorSet {
                        dst_set: descriptor_set,
                        dst_binding: 0,
                        dst_array_element: 0,
                        descriptor_count: 1,
                        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                        p_buffer_info: &buffer_info,
                        ..Default::default()
                    }],
                    &[],
                );
            }
        }
        Ok(Self {
            buffer,
            block,
            descriptor_pool,
            descriptor_set,
            descriptor_set_layout,
        })
    }
}

/// Represents a Transform which is safe to perform write operations.
pub struct WritableTransform(Transform);

impl WritableTransform {
    /// Take ownership of a transform for safe write operations.
    pub(super) fn new(transform: Transform) -> Self {
        Self(transform)
    }

    /// Release the transform so it can be used for read operations.
    pub(super) fn release(self) -> Transform {
        self.0
    }

    /// Set the transformation matrix.
    pub fn set_transform(&mut self, transform: &Mat4f) {
        let transform_uniform_buffer =
            self.0.block.mapped_ptr as *mut TransformUniformBuffer;
        unsafe {
            (*transform_uniform_buffer)
                .transform
                .copy_from_slice(transform.as_slice());
        }
    }
}
