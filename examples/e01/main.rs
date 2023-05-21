use {
    anyhow::Result,
    ccthw::{
        application::{Application, GlfwWindow, Sketch},
        graphics::{
            g2d::G2D,
            vulkan_api::{TextureAtlas, TextureId},
        },
    },
};

#[derive(Default)]
struct HelloG2D {
    gasp_texture: TextureId,
}

impl Sketch for HelloG2D {
    fn new(_window: &mut GlfwWindow, g2d: &mut G2D) -> Result<Self> {
        g2d.clear_color = [0.0, 0.0, 0.0, 1.0];
        Ok(Self::default())
    }

    fn preload(&mut self, texture_atlas: &mut TextureAtlas) -> Result<()> {
        self.gasp_texture = texture_atlas.load_file("examples/e01/Gasp.png");
        Ok(())
    }

    fn update(&mut self, g2d: &mut G2D) -> Result<()> {
        g2d.rect(200.0, 200.0, 200.0, 200.0, self.gasp_texture);
        g2d.rect(-200.0, -200.0, 200.0, 200.0, self.gasp_texture);
        Ok(())
    }
}

fn main() -> Result<()> {
    Application::<HelloG2D>::run()
}
