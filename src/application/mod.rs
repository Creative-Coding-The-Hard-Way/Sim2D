//! Provides structures for running a stateful single-window GLFW application.

mod loading_sketch;
mod logging;
mod timer;

use {
    self::timer::Timer,
    crate::{
        graphics::{NewAssetsCommand, Renderer, G2D},
        sim2d::Sim2D,
        DynSketch, Sketch,
    },
    anyhow::Result,
    glfw::WindowEvent,
    loading_sketch::LoadingSketch,
    std::{sync::mpsc::Receiver, thread::JoinHandle},
};

type PreloadJoinHandle = JoinHandle<Result<(DynSketch, NewAssetsCommand)>>;

pub use crate::window::{GlfwWindow, WindowState};

/// Every sketch is comprised of a State type and a GLFW window.
/// Sketches automatically pause if they are minimized or the window is
/// resized such that there is no drawing area.
pub struct Application {
    loading_join_handle: Option<PreloadJoinHandle>,

    sim: Sim2D,
    loading_sketch: LoadingSketch,
    sketch: DynSketch,

    paused: bool,
    timer: Timer,

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
        S: Sketch + Send + 'static,
    {
        crate::application::logging::setup();
        let window_title = std::any::type_name::<S>();
        let (window, event_receiver) = GlfwWindow::new(window_title)?;

        Self::new(window, sketch)?.main_loop(event_receiver)
    }
}

// Private API

impl Application {
    /// Create a new running application.
    fn new<S>(window: GlfwWindow, sketch: S) -> Result<Self>
    where
        S: Sketch + Send + 'static,
    {
        let render_device = unsafe { window.create_render_device()? };
        let mut renderer =
            Renderer::new(render_device, window.get_framebuffer_size())?;

        let mut asset_loader = renderer.new_asset_loader();
        let mut loading = LoadingSketch::default();
        loading.preload(&mut asset_loader);

        renderer.load_new_assets(asset_loader.build_new_assets_command()?)?;

        let sim = Sim2D::new(G2D::new(), window.new_window_state());

        let mut app = Self {
            loading_join_handle: None,

            sim,
            loading_sketch: loading.clone(),
            sketch: Box::new(loading),

            timer: Timer::new(),
            paused: false,

            renderer,
            window,
        };

        app.spawn_load_thread(Box::new(sketch))?;

        Ok(app)
    }

    fn main_loop(
        mut self,
        event_receiver: Receiver<(f64, WindowEvent)>,
    ) -> Result<()> {
        while !(self.window.should_close()) {
            self.join_load_thread()?;

            self.window.glfw.poll_events();
            for (_, window_event) in glfw::flush_messages(&event_receiver) {
                self.handle_event(window_event)?;
            }
            self.window.update_window_to_match(&mut self.sim.w)?;

            if !self.paused {
                self.update()?;

                if !self.is_loading() {
                    if let Some(next_sketch) = self.sketch.load_sketch() {
                        self.spawn_load_thread(next_sketch)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn is_loading(&self) -> bool {
        self.loading_join_handle.is_some()
    }

    fn spawn_load_thread(&mut self, mut sketch: DynSketch) -> Result<()> {
        self.sketch = Box::new(self.loading_sketch.clone());
        self.sketch.setup(&mut self.sim);
        self.window.update_window_to_match(&mut self.sim.w)?;

        let mut asset_loader = self.renderer.new_asset_loader();
        let join_handle: PreloadJoinHandle = std::thread::spawn(
            move || -> Result<(DynSketch, NewAssetsCommand)> {
                sketch.preload(&mut asset_loader);
                Ok((sketch, asset_loader.build_new_assets_command()?))
            },
        );

        debug_assert!(self.loading_join_handle.is_none());
        self.loading_join_handle = Some(join_handle);

        Ok(())
    }

    fn join_load_thread(&mut self) -> Result<()> {
        let is_finished = self
            .loading_join_handle
            .as_ref()
            .map_or(false, |handle| handle.is_finished());

        if is_finished {
            let handle = self.loading_join_handle.take().unwrap();
            let (sketch, new_assets_cmd) = handle.join().unwrap()?;
            self.sketch = sketch;
            self.renderer.load_new_assets(new_assets_cmd)?;

            self.sim.g = G2D::new();
            self.sketch.setup(&mut self.sim);
            self.window.update_window_to_match(&mut self.sim.w)?;
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
            WindowEvent::Pos(_, _) => {
                self.timer.reset_frame_time();
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
