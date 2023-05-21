use {
    anyhow::Result,
    sim2d::{
        application::{Application, GlfwWindow, Sketch},
        graphics::{
            g2d::G2D,
            vulkan_api::{TextureAtlas, TextureId},
        },
    },
};

#[derive(Default)]
struct HelloG2D {
    gasp: TextureId,
    t: f32,
}

impl Sketch for HelloG2D {
    fn new(_window: &mut GlfwWindow, g2d: &mut G2D) -> Result<Self> {
        g2d.clear_color = [0.0, 0.0, 0.0, 1.0];
        Ok(Self::default())
    }

    fn preload(&mut self, texture_atlas: &mut TextureAtlas) -> Result<()> {
        self.gasp = texture_atlas.load_file("examples/e01/Gasp.png");
        Ok(())
    }

    fn update(&mut self, g2d: &mut G2D) -> Result<()> {
        self.t += 0.001;

        let x = self.t.cos() * 500.0;
        let y = self.t.sin() * 500.0;
        g2d.texture = TextureId::no_texture();
        g2d.line(0.0, 0.0, x, y);

        g2d.texture = self.gasp;
        g2d.rect_centered(x, y, 200.0, 200.0);

        Ok(())
    }
}

fn main() -> Result<()> {
    Application::<HelloG2D>::run()
}
