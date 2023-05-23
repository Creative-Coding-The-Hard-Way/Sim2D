mod timer;
mod window_state;

use {crate::graphics::g2d::G2D, std::time::Duration};

pub use self::{timer::Timer, window_state::WindowState};

/// The simulation entrypoint.
pub struct Sim2D {
    pub g: G2D,
    pub w: WindowState,

    delta_time: f32,
    update_timer: Timer,
}

// Public API
// ----------

impl Sim2D {
    pub fn dt(&self) -> f32 {
        self.delta_time
    }
}

// Private API
// -----------

impl Sim2D {
    /// Create a new Simulation.
    pub(crate) fn new(g: G2D, w: WindowState) -> Self {
        Self {
            g,
            w,
            delta_time: 0.0,
            update_timer: Timer::new_with_report_rate(Duration::from_secs(1)),
        }
    }

    /// Reset the last tick instant without computing the delta time.
    pub(crate) fn reset_timer(&mut self) {
        self.update_timer.tick()
    }

    /// Tick the simulation's clock.
    pub(crate) fn update_timer(&mut self) {
        self.delta_time = self.update_timer.tock();
        if let Some(duration) = self.update_timer.report_average() {
            let secs = duration.as_secs_f32();
            log::info!(
                "Frame Time: {}ms\nFPS: {}",
                (secs * 1000.0 * 100.0).floor() / 100.0,
                (1.0 / secs).floor(),
            );
        }
        self.update_timer.tick();
    }
}
