use {
    super::super::Vertex,
    crate::graphics::vulkan::{memory, raii, render_context::RenderContext},
    anyhow::Result,
    ash::vk,
};

pub struct VertexBuffer {
    pub device_buffer_addr: vk::DeviceAddress,
    pub vertex_buffer: raii::BufferArc,
    pub memory: memory::OwnedBlock,
}

impl VertexBuffer {
    pub fn new(rc: &RenderContext) -> Result<Self> {
        let vertex_capacity = 3;
        let size_in_bytes =
            (std::mem::size_of::<Vertex>() * vertex_capacity) as u64;
        let vertex_buffer = {
            let create_info = vk::BufferCreateInfo {
                size: size_in_bytes,
                usage: vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &rc.graphics_queue_index,
                ..Default::default()
            };
            raii::Buffer::new(rc.device.clone(), &create_info)?
        };
        let block = {
            let requirements = unsafe {
                rc.device.get_buffer_memory_requirements(vertex_buffer.raw)
            };
            rc.allocator.allocate(
                requirements,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
                vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            )?
        };
        unsafe {
            rc.device.bind_buffer_memory(
                vertex_buffer.raw,
                block.memory.raw,
                0,
            )?;
        }
        let device_buffer_addr = {
            let info = vk::BufferDeviceAddressInfo {
                buffer: vertex_buffer.raw,
                ..Default::default()
            };
            unsafe { rc.device.get_buffer_device_address(&info) }
        };
        Ok(Self {
            vertex_buffer,
            device_buffer_addr,
            memory: block,
        })
    }

    /// Write vertex data into the buffer.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - It is not safe to write vertex data while the buffer is in use by the
    ///   GPU.
    pub unsafe fn write_vertex_data(&mut self, vertices: &[Vertex; 3]) {
        let slice = std::slice::from_raw_parts_mut(
            self.memory.mapped_ptr as *mut Vertex,
            3,
        );
        slice.copy_from_slice(vertices);
    }
}
