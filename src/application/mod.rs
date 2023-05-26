//! Provides structures for running a stateful single-window GLFW application.

mod glfw_window;
mod logging;
mod sketch;

use {
    crate::{
        graphics::{g2d::G2D, Renderer},
        sim2d::{Sim2D, WindowState},
    },
    anyhow::Result,
    glfw::WindowEvent,
};

pub use self::{glfw_window::GlfwWindow, sketch::Sketch};

/// Every sketch is comprised of a State type and a GLFW window.
/// Sketches automatically pause if they are minimized or the window is
/// resized such that there is no drawing area.
pub struct Application {
    sketch: Box<dyn Sketch>,

    sim: Sim2D,
    renderer: Renderer,

    paused: bool,
    window: GlfwWindow,
}

// Public API

impl Application {
    /// Create and run the Application until the window is closed.
    ///
    /// The window title is just the Application state struct's type name.
    pub fn run<S>(sketch: S) -> Result<()>
    where
        S: Sketch + 'static,
    {
        self::logging::setup();
        let window_title = std::any::type_name::<S>();
        Self::new(window_title, sketch)?.main_loop()
    }
}

// Private API

impl Application {
    /// Create a new running application.
    fn new<S>(window_title: impl AsRef<str>, mut sketch: S) -> Result<Self>
    where
        S: Sketch + 'static,
    {
        let mut window = GlfwWindow::new(window_title)?;
        let render_device = unsafe { window.create_render_device()? };

        let mut renderer =
            Renderer::new(render_device, window.get_framebuffer_size())?;

        let mut sim =
            Sim2D::new(G2D::new(), WindowState::from_glfw_window(&window));

        sketch.preload(renderer.texture_atlas_mut());
        renderer.reload_textures()?;

        sketch.setup(&mut sim);
        sim.w.update_window_to_match(&mut window)?;

        Ok(Self {
            sketch: Box::new(sketch),

            sim,
            renderer,

            paused: false,
            window,
        })
    }

    fn main_loop(mut self) -> Result<()> {
        self.sketch.setup(&mut self.sim);
        self.sim.reset_timer();

        let event_receiver = self.window.event_receiver.take().unwrap();
        while !(self.window.should_close()) {
            self.window.glfw.poll_events();
            for (_, window_event) in glfw::flush_messages(&event_receiver) {
                self.handle_event(window_event)?;
            }
            self.sim.w.update_window_to_match(&mut self.window)?;

            if !self.paused {
                self.update()?
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, window_event: WindowEvent) -> Result<()> {
        self.sim.w.handle_event(&window_event)?;
        match window_event {
            WindowEvent::MouseButton(_, glfw::Action::Press, _) => {
                self.sketch.mouse_pressed(&mut self.sim);
            }
            WindowEvent::MouseButton(_, glfw::Action::Release, _) => {
                self.sketch.mouse_released(&mut self.sim);
            }
            WindowEvent::Key(key, _scancode, glfw::Action::Press, _) => {
                self.sketch.key_pressed(&mut self.sim, key);
            }
            WindowEvent::Key(key, _scancode, glfw::Action::Release, _) => {
                self.sketch.key_released(&mut self.sim, key);
            }
            WindowEvent::CursorPos(_, _) => {
                self.sketch.mouse_moved(&mut self.sim);
            }
            WindowEvent::FramebufferSize(width, height) => {
                let was_paused = self.paused;
                self.paused = width == 0 || height == 0;

                if was_paused && !self.paused {
                    // reset the tick when unpaused
                    self.sim.reset_timer();
                    log::warn!("Unpaused");
                }

                if !self.paused {
                    self.renderer.rebuild_swapchain((width, height))?
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.sim.update_timer();
        self.sketch.update(&mut self.sim);

        self.renderer
            .render(self.window.get_framebuffer_size(), &mut self.sim.g)?;

        Ok(())
    }
}
