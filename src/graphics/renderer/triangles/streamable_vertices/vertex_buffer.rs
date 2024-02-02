use {
    super::super::Vertex,
    crate::graphics::vulkan::{
        memory::DeviceAllocator, render_context::RenderContext,
    },
    anyhow::Result,
    ash::vk,
};

#[derive(Copy, Clone)]
pub struct VertexBuffer {
    pub vertex_buffer: vk::Buffer,
    pub device_buffer_addr: vk::DeviceAddress,
    pub memory: vk::DeviceMemory,
    mapped_ptr: *mut std::ffi::c_void,
}

/// # Safety
///
/// Safe because:
/// - The VertexBuffer includes a mapped pointer to the underlying device
///   memory. It is okay to send the pointer to other threads, but the
///   application must still synchronize access to the underlying GPU resources.
unsafe impl Send for VertexBuffer {}
unsafe impl Sync for VertexBuffer {}

impl VertexBuffer {
    pub fn new(
        rc: &RenderContext,
        allocator: &DeviceAllocator,
    ) -> Result<Self> {
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
            unsafe { rc.device.create_buffer(&create_info, None)? }
        };
        let memory = {
            let requirements = unsafe {
                rc.device.get_buffer_memory_requirements(vertex_buffer)
            };
            allocator.allocate_memory(
                rc,
                requirements,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
                vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            )?
        };
        unsafe {
            rc.device.bind_buffer_memory(vertex_buffer, memory, 0)?;
        }
        let mapped_ptr = unsafe {
            rc.device.map_memory(
                memory,
                0,
                size_in_bytes,
                vk::MemoryMapFlags::empty(),
            )?
        };
        let device_buffer_addr = {
            let info = vk::BufferDeviceAddressInfo {
                buffer: vertex_buffer,
                ..Default::default()
            };
            unsafe { rc.device.get_buffer_device_address(&info) }
        };
        Ok(Self {
            vertex_buffer,
            device_buffer_addr,
            memory,
            mapped_ptr,
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
        let slice =
            std::slice::from_raw_parts_mut(self.mapped_ptr as *mut Vertex, 3);
        slice.copy_from_slice(vertices);
    }

    /// Destroy the vertex buffer and backing memory.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The VertexBuffer must not be used after calling this method.
    /// - This method must only be called once, even if the vertex buffer has
    ///   been cloned or copied many times.
    /// - It is unsafe to call this method while the GPU is still using the
    ///   vertex buffer.
    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        rc.device.destroy_buffer(self.vertex_buffer, None);
        rc.device.free_memory(self.memory, None);
    }
}
