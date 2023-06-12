use {
    crate::{math::Vec2, Sim2D},
    std::time::Duration,
};

/// Render the current FPS and simulation timing information to the top left
/// of the screen.
pub fn draw_fps_panel(sim: &mut Sim2D) {
    sim.g.text(
        Vec2::new(sim.w.width() * -0.5, sim.w.height() * 0.5),
        format!(
            indoc::indoc!(
                "
                |         FPS: {}
                |  Frame Time: {}ms
                |    Sim Time: {}ms
                | Render Time: {}ms
                "
            ),
            rounded(1.0 / sim.avg_frame_time().as_secs_f32()),
            get_rounded_ms(sim.avg_frame_time()),
            get_rounded_ms(sim.avg_sim_time()),
            get_rounded_ms(sim.avg_render_time()),
        ),
    );
}

/// Round a floating point number to have 2 decimal places.
fn rounded(f: f32) -> f32 {
    (f * 100.0).round() / 100.0
}

fn get_rounded_ms(d: &Duration) -> f32 {
    rounded(d.as_secs_f32() * 1000.0)
}
