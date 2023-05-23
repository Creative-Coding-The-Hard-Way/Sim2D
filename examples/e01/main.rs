use {
    anyhow::Result,
    sim2d::{
        application::{Application, Sketch},
        graphics::vulkan_api::{TextureAtlas, TextureId},
        math::Vec2,
        sim2d::Sim2D,
    },
};

#[derive(Default)]
struct HelloG2D {
    gasp: TextureId,
}

impl Sketch for HelloG2D {
    fn new(sim: &mut Sim2D) -> Result<Self> {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];

        Ok(Self {
            ..Default::default()
        })
    }

    fn preload(&mut self, texture_atlas: &mut TextureAtlas) -> Result<()> {
        self.gasp = texture_atlas.load_file("examples/e01/Gasp.png");
        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) -> Result<()> {
        sim.g.texture = self.gasp;

        sim.g
            .rect_centered(sim.w.mouse_pos(), Vec2::new(200.0, 200.0), 0.0);

        Ok(())
    }
}

fn main() -> Result<()> {
    Application::<HelloG2D>::run()
}
