use {
    anyhow::Result,
    sim2d::{
        application::Application,
        graphics::{TextureAtlas, TextureId},
        math::Vec2,
        DynSketch, Sim2D, Sketch,
    },
    std::time::Duration,
};

/// A slow-loading sketch to demo the loading screen.
///
/// Clicking the mouse will respawn the sketch from scratch for loading.
#[derive(Default)]
struct SlowLoad {
    wants_reload: bool,
}

impl Sketch for SlowLoad {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];
        sim.w.resize(1000.0, 1000.0);
        self.wants_reload = false;
    }

    fn preload(&mut self, _texture_atlas: &mut TextureAtlas) {
        std::thread::sleep(Duration::from_secs(5));
    }

    fn mouse_released(&mut self, _sim: &mut Sim2D) {
        self.wants_reload = true;
    }

    fn load_sketch(&mut self) -> Option<DynSketch> {
        if self.wants_reload {
            Some(Box::<SlowLoad>::default())
        } else {
            None
        }
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.texture = TextureId::no_texture();
        sim.g
            .rect_centered(sim.w.mouse_pos(), Vec2::new(100.0, 100.0), 0.0)
    }
}

fn main() -> Result<()> {
    Application::run(SlowLoad::default())
}
