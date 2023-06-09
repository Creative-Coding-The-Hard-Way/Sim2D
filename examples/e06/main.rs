use {
    anyhow::Result,
    sim2d::{
        application::Application,
        graphics::{AssetLoader, FontId},
        math::Vec2,
        Sim2D, Sketch,
    },
};

/// A slow-loading sketch to demo the loading screen.
///
/// Clicking the mouse will respawn the sketch from scratch for loading.
#[derive(Default)]
struct TextRendering {
    my_font: FontId,
}

impl Sketch for TextRendering {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];
        sim.w.resize(1000.0, 1000.0);
    }

    fn preload(&mut self, loader: &mut AssetLoader) -> Result<()> {
        self.my_font =
            loader.load_font_file("examples/e06/NotoSans-Regular.ttf", 32.0)?;
        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];

        sim.g.font = FontId::default_font();
        sim.g.text(Vec2::new(0.0, 0.0), "hello world");

        sim.g.text(
            Vec2::new(sim.w.width() * -0.5, sim.w.height() * 0.5),
            format!(
                indoc::indoc!(
                    "
                    FPS: {}
                    Frame Time: {}ms
                    "
                ),
                1.0 / sim.dt(),
                (sim.dt() * 1000.0 * 10.0).ceil() / 10.0,
            ),
        );

        sim.g.font = self.my_font;
        sim.g.text(
            sim.w.mouse_pos(),
            indoc::indoc!(
                "
                My Custom font
                Something else
                AOEU
                ^-^
                "
            ),
        );
    }
}

fn main() -> Result<()> {
    Application::run(TextRendering::default())
}
