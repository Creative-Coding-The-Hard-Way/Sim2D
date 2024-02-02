mod vertex_buffer;

pub use vertex_buffer::{Vertex, VertexBuffer};
use {
    anyhow::{Context, Result},
    sim2d::graphics::vulkan::{
        memory::DeviceAllocator, render_context::RenderContext,
    },
    std::collections::BTreeSet,
};

#[derive(Clone)]
struct InUse {
    vertex_buffer: VertexBuffer,
    used_by_frames: BTreeSet<usize>,
}

/// Maintains a queue of Vertex Buffers that can be used to present to the
/// screen.
pub struct StreamableVerticies {
    read_buffer: InUse,
    free: Vec<VertexBuffer>,
    in_use: Vec<InUse>,
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

        let read_buffer = InUse {
            vertex_buffer: *vertex_buffers.first().unwrap(),
            used_by_frames: BTreeSet::new(),
        };
        let free = vertex_buffers[1..].to_vec();

        Ok(Self {
            read_buffer,
            free,
            in_use: Vec::with_capacity(count),
            owned_vertex_buffers: vertex_buffers,
        })
    }

    pub fn try_get_writable_buffer(&mut self) -> Option<VertexBuffer> {
        self.free.pop()
    }

    /// This must be a buffer previously given by try_get_writable_buffer
    pub fn publish_update(&mut self, vertex_buffer: VertexBuffer) {
        self.in_use.push(self.read_buffer.clone());
        self.read_buffer = InUse {
            vertex_buffer,
            used_by_frames: BTreeSet::new(),
        };
    }

    pub fn get_read_buffer(&mut self, frame_index: usize) -> VertexBuffer {
        // mark the read buffer as being used by the frame
        self.read_buffer.used_by_frames.insert(frame_index);

        // Update the in-use resources to indicate they are no longer being
        // used by the frame.
        {
            for in_use in self.in_use.iter_mut() {
                in_use.used_by_frames.remove(&frame_index);
                if in_use.used_by_frames.is_empty() {
                    self.free.push(in_use.vertex_buffer);
                }
            }
            self.in_use
                .retain(|in_use| !in_use.used_by_frames.is_empty());
        }

        // Return the read buffer
        self.read_buffer.vertex_buffer
    }

    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        for vertex_buffer in &mut self.owned_vertex_buffers {
            vertex_buffer.destroy(rc);
        }
        self.free.clear();
        self.in_use.clear();
    }
}
