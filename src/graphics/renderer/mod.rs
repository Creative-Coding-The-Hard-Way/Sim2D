use {
    crate::{graphics::vulkan::render_context::RenderContext, trace},
    anyhow::{Context, Result},
    std::{sync::atomic::AtomicBool, thread::JoinHandle},
};

pub mod triangles;

pub enum RenderEvents {
    /// Send this event to the render thread when the framebuffer is resized.
    FramebufferResized(u32, u32),
}

pub trait Renderer {
    type ClientApi;

    fn new(rc: &RenderContext) -> Result<(Self, Self::ClientApi)>
    where
        Self: Sized;

    fn draw_frame(&mut self) -> Result<()>;

    fn shut_down(&mut self) -> Result<()>;
}

pub struct JoinableRenderer {
    thread_handle: JoinHandle<Result<()>>,
    running: AtomicBool,
}

impl JoinableRenderer {
    pub fn new<R>(rc: &RenderContext) -> Result<(Self, R::ClientApi)>
    where
        R: Renderer,
    {
        let (renderer, api) = R::new(rc)
            .with_context(trace!("Unable to create the renderer!"))?;
        todo!()
    }
}
