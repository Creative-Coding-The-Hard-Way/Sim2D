use crate::{application::WindowState, graphics::G2D};

/// The API entrypoint.
///
/// All interactions with the simulation's window and grahpics backend are
/// controlled by manipulating the Sim2D state.
pub struct Sim2D {
    pub g: G2D,
    pub w: WindowState,

    delta_time: f32,
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
        }
    }

    pub(crate) fn set_delta_time(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
    }
}
