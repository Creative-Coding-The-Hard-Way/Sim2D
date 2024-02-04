pub mod triangles;

pub enum RenderEvents {
    /// Send this event to the render thread when the framebuffer is resized.
    FramebufferResized(u32, u32),
}
