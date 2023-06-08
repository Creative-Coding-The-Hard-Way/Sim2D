use {
    ab_glyph::{Font, FontVec},
    anyhow::{Context, Result},
    sim2d::{
        application::Application,
        graphics::{AssetLoader, CachedFont},
        Sim2D, Sketch,
    },
};

/// A slow-loading sketch to demo the loading screen.
///
/// Clicking the mouse will respawn the sketch from scratch for loading.
#[derive(Default)]
struct TextRendering {
    bitmap_font: Option<CachedFont>,
}

impl Sketch for TextRendering {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];
        sim.w.resize(1000.0, 1000.0);
    }

    fn preload(&mut self, loader: &mut AssetLoader) -> Result<()> {
        let font_data = std::fs::read("examples/e06/NotoSans-Regular.ttf")
            .context("Unable to read font!")?;

        let font = FontVec::try_from_vec(font_data)?;
        let scaled_font = font.into_scaled(64.0);

        let cached_glyphs = "
            abcdefghijklmnopqrstuvwxyz
            ABCDEFGHIJKLMNOPQRSTUVWXYZ
            1234567890{}()[]*&^%$#@!+=
            -/\\\"'`;:<>.,_
            ";

        self.bitmap_font =
            Some(CachedFont::new(loader, scaled_font, cached_glyphs));

        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];

        let msg = format!(
            indoc::indoc!(
                "
                Frame Time: {}ms
                FPS: {}

                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
                eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
                enim ad minim veniam, quis nostrud exercitation ullamco laboris
                nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor
                in reprehenderit in voluptate velit esse cillum dolore eu fugiat
                nulla pariatur. Excepteur sint occaecat cupidatat non proident,
                sunt in culpa qui officia deserunt mollit anim id est laborum.
                "
            ),
            (sim.dt() * 1000.0 * 10.0).ceil() / 10.0,
            (10.0 / sim.dt()).ceil() / 10.0
        );

        self.bitmap_font.as_ref().unwrap().draw_text(
            sim,
            sim.w.mouse_pos(),
            msg,
        );
    }
}

fn main() -> Result<()> {
    Application::run(TextRendering::default())
}
