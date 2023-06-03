use {
    ab_glyph::{Font, FontVec, ScaleFont},
    anyhow::{Context, Result},
    image::Rgba,
    sim2d::{
        application::Application,
        graphics::{AssetLoader, Image},
        math::Vec2,
        Sim2D, Sketch,
    },
};

fn layout_text<F, SF>(font: SF, text: impl AsRef<str>)
where
    F: Font,
    SF: ScaleFont<F>,
{
    let mut cursor = ab_glyph::point(0.0, font.ascent());
    for c in text.as_ref().chars() {
        //
        let mut glyph = font.scaled_glyph(c);
        //if let Some(previous) = last_glyph.take() {
        //    caret.x += font.kern(previous.id, glyph.id);
        //}
    }
    todo!()
}

/// A slow-loading sketch to demo the loading screen.
///
/// Clicking the mouse will respawn the sketch from scratch for loading.
#[derive(Default)]
struct TextRendering {
    font_atlas: Image,
}

impl Sketch for TextRendering {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];
        sim.w.resize(1000.0, 1000.0);
    }

    fn preload(&mut self, _loader: &mut AssetLoader) -> Result<()> {
        let font_data = std::fs::read("examples/e06/NotoSans-Regular.ttf")
            .context("Unable to read font!")?;

        let font = FontVec::try_from_vec(font_data)?;
        let scaled_font = font.as_scaled(64.0);

        let mut m = scaled_font.scaled_glyph('j');
        m.position = ab_glyph::point(0.0, scaled_font.ascent());
        let outline_m = scaled_font.outline_glyph(m.clone()).unwrap();

        log::info!(
            indoc::indoc!(
                "
                Info about glyph
                position: {:?}
                X: [{}, {}],
                Y: [{}, {}],
                "
            ),
            m.position,
            outline_m.px_bounds().min.x,
            outline_m.px_bounds().max.x,
            outline_m.px_bounds().min.y,
            outline_m.px_bounds().max.y,
        );

        let w = 2;
        let h = 2;

        let mut img = image::DynamicImage::new_rgba8(w, h).to_rgba8();
        img.fill(0);

        // Top left
        img.put_pixel(0, 0, Rgba::from([10, 10, 10, 255]));

        //// Top right - blue
        img.put_pixel(1, 0, Rgba::from([0, 0, 255, 255]));

        //// bottom left - white
        img.put_pixel(0, 1, Rgba::from([255, 255, 255, 255]));

        //// bottom right - green
        img.put_pixel(1, 1, Rgba::from([0, 255, 0, 255]));

        let bounds_m = outline_m.px_bounds();
        outline_m.draw(|x, y, _v| {
            let px = x + bounds_m.min.x as u32;
            let py = y + bounds_m.min.y as u32;
            log::info!("write value at: {}, {}", px, py);

            //let px = img.get_pixel_mut(
            //    x + bounds_m.min.x as u32,
            //    y + bounds_m.min.y as u32,
            //);
            //*px = image::Rgba::from([
            //    255,
            //    255,
            //    255,
            //    (v * 255.0).round().min(255.0) as u8,
            //]);
        });

        self.font_atlas = _loader.load_image(img, true);

        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.image = self.font_atlas;
        sim.g.rect_centered(
            Vec2::new(0.0, 0.0),
            //Vec2::new(self.font_atlas.width(), self.font_atlas.height()),
            Vec2::new(200.0, 200.0),
            0.0,
        );
    }
}

fn main() -> Result<()> {
    Application::run(TextRendering::default())
}
