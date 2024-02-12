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

pub struct InterpolatedPrimitivesApi {
    vertex_buffer_client: AsyncNBufferClient<VertexBuffer>,
    transform_client: AsyncNBufferClient<Transform>,
    framebuffer_size_sender: Sender<(u32, u32)>,
}

impl InterpolatedPrimitivesApi {
    /// Create a new instance of the API. Only ever called by the
    /// InterpolatedPrimitivesRenderer.
    pub(super) fn new(
        vertex_buffer_client: AsyncNBufferClient<VertexBuffer>,
        transform_client: AsyncNBufferClient<Transform>,
        framebuffer_size_sender: Sender<(u32, u32)>,
    ) -> Self {
        Self {
            vertex_buffer_client,
            transform_client,
            framebuffer_size_sender,
        }
    }

    /// Set the current projection matrix. Can be a no-op if the renderer is not
    /// ready for a new matrix yet.
    pub fn set_projection(&mut self, matrix: [[f32; 4]; 4]) -> Result<()> {
        if let Some(transform) =
            self.transform_client.wait_for_free_resource()?
        {
            let mut writable = WritableTransform::new(transform);
            writable.set_transform(matrix);
            self.transform_client
                .make_resource_current(writable.release())?;
        }
        Ok(())
    }

    /// Publish new vertices to the renderer. Can be a no-op if there are no
    /// vertex buffers available.
    pub fn publish_vertices(
        &mut self,
        rc: &RenderContext,
        vertices: &[Vertex],
    ) -> Result<()> {
        if let Some(vertex_buffer) =
            self.vertex_buffer_client.wait_for_free_resource()?
        {
            let mut writable_vertices =
                WritableVertexBuffer::new(vertex_buffer);
            writable_vertices.write_vertex_data(rc, vertices)?;
            self.vertex_buffer_client
                .make_resource_current(writable_vertices.release())?;
        }
        Ok(())
    }

    /// Let the renderer know the current framebuffer size.
    pub fn framebuffer_resized(
        &mut self,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        self.framebuffer_size_sender.send(framebuffer_size)?;
        Ok(())
    }
}
