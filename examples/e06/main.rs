use {
    anyhow::Result,
    sim2d::{
        application::Application, graphics::AssetLoader, math::Vec2, Sim2D,
        Sketch,
    },
};

/// A slow-loading sketch to demo the loading screen.
///
/// Clicking the mouse will respawn the sketch from scratch for loading.
#[derive(Default)]
struct TextRendering {}

impl Sketch for TextRendering {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];
        sim.w.resize(1000.0, 1000.0);
    }

    fn preload(&mut self, _loader: &mut AssetLoader) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g
            .rect_centered(sim.w.mouse_pos(), Vec2::new(100.0, 100.0), 0.0)
    }
}

fn main() -> Result<()> {
    Application::run(TextRendering::default())
}
