mod vertex_buffer;

pub use vertex_buffer::VertexBuffer;
use {
    crate::graphics::vulkan::{
        memory::DeviceAllocator, render_context::RenderContext, sync::NBuffer,
    },
    anyhow::{Context, Result},
};

/// Maintains a queue of Vertex Buffers that can be used to present to the
/// screen.
pub struct StreamableVerticies {
    sync: NBuffer<VertexBuffer>,
    owned_vertex_buffers: Vec<VertexBuffer>,
}

impl StreamableVerticies {
    pub fn new(
        rc: &RenderContext,
        allocator: &DeviceAllocator,
        count: usize,
    ) -> Result<Self> {
        assert!(count >= 2, "Must be at least double buffered!");

        let mut vertex_buffers = Vec::with_capacity(count);
        for _ in 0..count {
            let vertex_buffer = VertexBuffer::new(rc, allocator)
                .context("Unable to create vertex buffer!")?;
            vertex_buffers.push(vertex_buffer);
        }

        let sync = NBuffer::new(&vertex_buffers);

        Ok(Self {
            sync,
            owned_vertex_buffers: vertex_buffers,
        })
    }

    pub fn try_get_writable_buffer(&mut self) -> Option<VertexBuffer> {
        self.sync.try_get_free_resource()
    }

    /// This must be a buffer previously given by try_get_writable_buffer
    pub fn publish_update(&mut self, vertex_buffer: VertexBuffer) {
        self.sync.make_current(vertex_buffer);
    }

    pub fn get_read_buffer(&mut self, frame_index: usize) -> VertexBuffer {
        self.sync.get_current(frame_index)
    }

    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        for vertex_buffer in &mut self.owned_vertex_buffers {
            vertex_buffer.destroy(rc);
        }
        self.sync.destroy();
    }
}
