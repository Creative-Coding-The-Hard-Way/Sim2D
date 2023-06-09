use {
    crate::{application::WindowState, graphics::G2D},
    std::time::Duration,
};

/// The API entrypoint.
///
/// All interactions with the simulation's window and grahpics backend are
/// controlled by manipulating the Sim2D state.
pub struct Sim2D {
    pub g: G2D,
    pub w: WindowState,

    pub(crate) delta_time: f32,
    pub(crate) avg_frame_time: Duration,
    pub(crate) avg_sim_time: Duration,
    pub(crate) avg_render_time: Duration,
}

// Public API
// ----------

impl Sim2D {
    pub fn dt(&self) -> f32 {
        self.delta_time
    }

    pub fn avg_frame_time(&self) -> &Duration {
        &self.avg_frame_time
    }

    pub fn avg_sim_time(&self) -> &Duration {
        &self.avg_sim_time
    }

    pub fn avg_render_time(&self) -> &Duration {
        &self.avg_render_time
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
            avg_frame_time: Duration::default(),
            avg_sim_time: Duration::default(),
            avg_render_time: Duration::default(),
        }
    }
}
