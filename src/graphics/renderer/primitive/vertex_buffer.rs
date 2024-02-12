use {
    crate::graphics::vulkan::{
        memory, raii,
        render_context::RenderContext,
        sync::{AsyncNBuffer, AsyncNBufferClient},
    },
    anyhow::Result,
    ash::vk,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub rgba: [f32; 4],
    pub pos: [f32; 2],
    pub vel: [f32; 2],
}

impl Vertex {
    /// Create a new vertex out of things that turn into float arrays.
    pub fn new(
        pos: impl Into<[f32; 2]>,
        vel: impl Into<[f32; 2]>,
        rgba: impl Into<[f32; 4]>,
    ) -> Self {
        Vertex {
            rgba: rgba.into(),
            pos: pos.into(),
            vel: vel.into(),
        }
    }
}

/// All of the resources required to keep track of a vertex buffer.
pub struct VertexBuffer {
    pub vertex_count: u32,
    pub buffer_address: vk::DeviceAddress,
    pub buffer: raii::BufferArc,
    pub block: memory::OwnedBlock,
}

/// Represents a VertexBuffer that is safe to write.
pub struct WritableVertexBuffer(VertexBuffer);

impl WritableVertexBuffer {
    pub(super) fn new(vertex_buffer: VertexBuffer) -> Self {
        Self(vertex_buffer)
    }

    pub(super) fn release(self) -> VertexBuffer {
        self.0
    }

    /// Write vertex data into the buffer. Reallocates GPU memory if there isn't
    /// sufficient space for the provided vertices.
    pub fn write_vertex_data(
        &mut self,
        rc: &RenderContext,
        vertices: &[Vertex],
    ) -> Result<()> {
        if vertices.len() > self.0.vertex_capacity() {
            self.0.grow_vertex_capacity(rc, vertices.len() as u32)?;
        }

        unsafe {
            let slice = std::slice::from_raw_parts_mut(
                self.0.block.mapped_ptr as *mut Vertex,
                vertices.len(),
            );
            slice.copy_from_slice(vertices);
        }
        self.0.vertex_count = vertices.len() as u32;
        Ok(())
    }
}

impl VertexBuffer {
    pub fn create_n_buffered(
        rc: &RenderContext,
        count: usize,
    ) -> Result<(AsyncNBuffer<Self>, AsyncNBufferClient<Self>)> {
        let mut vertex_buffers = Vec::with_capacity(count);
        for _ in 0..count {
            vertex_buffers.push(Self::new(rc)?);
        }
        AsyncNBuffer::new(vertex_buffers)
    }

    fn new(rc: &RenderContext) -> Result<Self> {
        let initial_capacity =
            (memory::MB as u32 / std::mem::size_of::<Vertex>() as u32) + 1;
        let (buffer, block, buffer_address) =
            create_vertex_buffer(rc, initial_capacity)?;
        Ok(Self {
            buffer,
            buffer_address,
            block,
            vertex_count: 0,
        })
    }

    // Private API

    fn vertex_capacity(&self) -> usize {
        self.block.size_in_bytes as usize / std::mem::size_of::<Vertex>()
    }

    /// Attempts to grow the capacity of this buffer by a fixed ratio.
    fn grow_vertex_capacity(
        &mut self,
        rc: &RenderContext,
        required_capacity: u32,
    ) -> Result<()> {
        let desired_capacity =
            required_capacity.max((self.vertex_capacity() * 2) as u32);
        let (buffer, block, address) =
            create_vertex_buffer(rc, desired_capacity)?;
        self.buffer = buffer;
        self.block = block;
        self.buffer_address = address;
        Ok(())
    }
}

fn create_vertex_buffer(
    rc: &RenderContext,
    vertex_capacity: u32,
) -> Result<(raii::BufferArc, memory::OwnedBlock, vk::DeviceAddress)> {
    let size_in_bytes =
        vertex_capacity as usize * std::mem::size_of::<Vertex>();
    let vertex_buffer = {
        let create_info = vk::BufferCreateInfo {
            size: size_in_bytes as u64,
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
        rc.device
            .bind_buffer_memory(vertex_buffer.raw, block.memory.raw, 0)?;
    }
    let device_buffer_addr = {
        let info = vk::BufferDeviceAddressInfo {
            buffer: vertex_buffer.raw,
            ..Default::default()
        };
        unsafe { rc.device.get_buffer_device_address(&info) }
    };
    Ok((vertex_buffer, block, device_buffer_addr))
}
