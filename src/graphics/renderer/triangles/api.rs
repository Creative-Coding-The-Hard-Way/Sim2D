use {
    super::{
        transform::{Transform, WritableTransform},
        vertex_buffer::VertexBuffer,
        Vertex, WritableVertexBuffer,
    },
    crate::graphics::vulkan::{
        render_context::RenderContext, sync::AsyncNBufferClient,
    },
    anyhow::Result,
    std::sync::mpsc::Sender,
};

pub struct TrianglesApi {
    vertex_buffer_client: AsyncNBufferClient<VertexBuffer>,
    transform_client: AsyncNBufferClient<Transform>,
    framebuffer_size_sender: Sender<(u32, u32)>,
    pub vertices: Vec<Vertex>,
}

impl TrianglesApi {
    pub(super) fn new(
        vertex_buffer_client: AsyncNBufferClient<VertexBuffer>,
        transform_client: AsyncNBufferClient<Transform>,
        framebuffer_size_sender: Sender<(u32, u32)>,
    ) -> Self {
        Self {
            vertex_buffer_client,
            transform_client,
            framebuffer_size_sender,
            vertices: Vec::with_capacity(10_000),
        }
    }

    pub fn vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }

    pub fn set_projection(&mut self, matrix: [[f32; 4]; 4]) -> Result<()> {
        let transform = self.transform_client.wait_for_free_resource()?;
        let mut writable = WritableTransform::new(transform);
        writable.set_transform(matrix);
        self.transform_client
            .make_resource_current(writable.release())
    }

    pub fn publish_vertices(&mut self, rc: &RenderContext) -> Result<()> {
        let vertex_buffer =
            self.vertex_buffer_client.wait_for_free_resource()?;
        let mut writable_vertices = WritableVertexBuffer::new(vertex_buffer);
        writable_vertices.write_vertex_data(rc, &self.vertices)?;
        self.vertices.clear();
        self.vertex_buffer_client
            .make_resource_current(writable_vertices.release())
    }

    pub fn framebuffer_resized(
        &mut self,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        self.framebuffer_size_sender.send(framebuffer_size)?;
        Ok(())
    }
}
