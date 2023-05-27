//! Provides structures for running a stateful single-window GLFW application.

mod logging;
mod timer;

use {
    self::timer::Timer,
    crate::{
        graphics::{Renderer, G2D},
        sim2d::Sim2D,
        Sketch,
    },
    anyhow::Result,
    glfw::WindowEvent,
    std::sync::mpsc::Receiver,
};

pub use crate::window::{GlfwWindow, WindowState};

/// Every sketch is comprised of a State type and a GLFW window.
/// Sketches automatically pause if they are minimized or the window is
/// resized such that there is no drawing area.
pub struct Application {
    sim: Sim2D,
    sketch: Box<dyn Sketch>,

    paused: bool,
    timer: Timer,
    event_receiver: Option<Receiver<(f64, WindowEvent)>>,

    renderer: Renderer,
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
        crate::application::logging::setup();
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
        let (mut window, event_receiver) = GlfwWindow::new(window_title)?;
        let render_device = unsafe { window.create_render_device()? };
        let mut renderer =
            Renderer::new(render_device, window.get_framebuffer_size())?;

        let mut sim = Sim2D::new(G2D::new(), window.new_window_state());

        sketch.preload(renderer.texture_atlas_mut());
        renderer.reload_textures()?;

        sketch.setup(&mut sim);
        window.update_window_to_match(&mut sim.w)?;

        Ok(Self {
            sim,
            sketch: Box::new(sketch),

            timer: Timer::new(),
            paused: false,
            event_receiver: Some(event_receiver),

            renderer,
            window,
        })
    }

    fn main_loop(mut self) -> Result<()> {
        let event_receiver = self.event_receiver.take().unwrap();
        while !(self.window.should_close()) {
            self.window.glfw.poll_events();
            for (_, window_event) in glfw::flush_messages(&event_receiver) {
                self.handle_event(window_event)?;
            }
            self.window.update_window_to_match(&mut self.sim.w)?;

            if !self.paused {
                self.update()?
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, window_event: WindowEvent) -> Result<()> {
        self.window.handle_event(&mut self.sim.w, &window_event)?;
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
                    self.timer.reset_frame_time();
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let total_dt = self.timer.frame_tick_tock();
        self.sim.set_delta_time(total_dt.as_secs_f32());

        self.timer.simulation_tick();
        self.sketch.update(&mut self.sim);
        self.timer.simulation_tock();

        self.timer.render_tick();
        self.renderer
            .render(self.window.get_framebuffer_size(), &mut self.sim.g)?;
        self.timer.render_tock();

        Ok(())
    }
}
