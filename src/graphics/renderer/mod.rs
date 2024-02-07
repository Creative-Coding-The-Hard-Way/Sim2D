use {
    crate::{graphics::vulkan::render_context::RenderContext, trace},
    anyhow::{Context, Result},
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread::JoinHandle,
    },
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
    thread_handle: Option<JoinHandle<Result<()>>>,
    running: Arc<AtomicBool>,
}

impl JoinableRenderer {
    pub fn new<R>(rc: &RenderContext) -> Result<(Self, R::ClientApi)>
    where
        R: Renderer + Send + 'static,
    {
        let (mut renderer, api) = R::new(rc)
            .with_context(trace!("Unable to create the renderer!"))?;

        let running = Arc::new(AtomicBool::new(true));

        let thread_handle = {
            let renderer_running = running.clone();
            std::thread::spawn(move || -> Result<()> {
                while renderer_running.load(Ordering::Relaxed) {
                    renderer.draw_frame()?;
                }
                renderer.shut_down()?;
                Ok(())
            })
        };

        Ok((
            Self {
                thread_handle: Some(thread_handle),
                running,
            },
            api,
        ))
    }

    pub fn shut_down(&mut self) -> Result<()> {
        self.running.store(false, Ordering::Relaxed);
        self.thread_handle.take().unwrap().join().unwrap()?;
        Ok(())
    }
}
