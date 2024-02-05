mod vertex_buffer;

pub use vertex_buffer::VertexBuffer;
use {
    crate::{
        graphics::vulkan::{
            render_context::RenderContext,
            sync::{AsyncNBuffer, AsyncNBufferClient},
        },
        trace,
    },
    anyhow::{Context, Result},
};

pub struct WritableVertices {
    client: AsyncNBufferClient<VertexBuffer>,
}

impl WritableVertices {
    pub fn wait_for_vertex_buffer(&self) -> Result<VertexBuffer> {
        self.client.wait_for_free_resource()
    }

    pub fn publish_update(&self, vertex_buffer: VertexBuffer) -> Result<()> {
        self.client.make_resource_current(vertex_buffer)
    }
}

/// Maintains a queue of Vertex Buffers that can be used to present to the
/// screen.
pub struct StreamableVerticies {
    sync: AsyncNBuffer<VertexBuffer>,
}

impl StreamableVerticies {
    pub fn new(
        rc: &RenderContext,
        count: usize,
    ) -> Result<(Self, WritableVertices)> {
        assert!(count >= 2, "Must be at least double buffered!");

        let mut vertex_buffers = Vec::with_capacity(count);
        for _ in 0..count {
            let vertex_buffer = VertexBuffer::new(rc)
                .context("Unable to create vertex buffer!")?;
            vertex_buffers.push(vertex_buffer);
        }

        let (sync, client) = AsyncNBuffer::new(vertex_buffers)?;

        let streamable = Self { sync };
        let client = WritableVertices { client };

        Ok((streamable, client))
    }

    pub fn get_read_buffer(
        &mut self,
        frame_index: usize,
    ) -> Result<&mut VertexBuffer> {
        self.sync.get_current(frame_index).with_context(trace!(
            "Unable to get readable buffer for the current frame!"
        ))
    }
}
