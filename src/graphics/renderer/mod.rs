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

pub mod primitive;

/// A Renderer is anything that can present frames and provides an API for
/// controlling on-screen geometry.
pub trait Renderer {
    /// A renderer's API may retain state (channels, thread-local caches,
    /// etc...). The API is created at the same time as the original
    /// renderer.
    type Api;

    /// A renderer can optionally be configured with parameters.
    type Parameters;

    /// Create a new instance of the renderer and its API.
    fn new(
        rc: &RenderContext,
        parameters: Self::Parameters,
    ) -> Result<(Self, Self::Api)>
    where
        Self: Sized;

    /// Draw a single frame.
    fn draw_frame(&mut self) -> Result<()>;

    /// Shut down the renderer.
    fn shut_down(&mut self) -> Result<()>;
}

/// An AsyncRenderer spawns a thread to execute the draw_frame loop.
///
/// The application interacts with the renderer exclusively via the API.
///
/// This struct derefs as the Renderer's api to simplify access.
pub struct AsyncRenderer<R: Renderer> {
    render_thread_handle: Option<JoinHandle<Result<()>>>,
    running: Arc<AtomicBool>,
    api: R::Api,
}

impl<R: Renderer + Send + 'static> AsyncRenderer<R> {
    /// Create a new Renderer.
    pub fn new(rc: &RenderContext, parameters: R::Parameters) -> Result<Self> {
        let (mut renderer, api) = R::new(rc, parameters)
            .with_context(trace!("Unable to create the renderer!"))?;

        let running = Arc::new(AtomicBool::new(true));

        let thread_handle = {
            let renderer_running = running.clone();
            std::thread::spawn(move || -> Result<()> {
                while renderer_running.load(Ordering::Relaxed) {
                    renderer.draw_frame()?;
                }
                log::trace!("Render Thread got shut down signal! Stopping...");
                renderer.shut_down()?;
                Ok(())
            })
        };

        Ok(Self {
            render_thread_handle: Some(thread_handle),
            running,
            api,
        })
    }

    /// Signal the render thread to shut down and attempt to join.
    pub fn shut_down(&mut self) -> Result<()> {
        log::trace!("Send shut down signal to the render thread.");
        self.running.store(false, Ordering::Release);

        log::trace!("Waiting for render thread to join...");
        self.render_thread_handle.take().map(JoinHandle::join);
        Ok(())
    }
}

impl<R: Renderer> std::ops::Deref for AsyncRenderer<R> {
    type Target = R::Api;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

impl<R: Renderer> std::ops::DerefMut for AsyncRenderer<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.api
    }
}
