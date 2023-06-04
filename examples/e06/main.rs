use {
    ab_glyph::{Font, FontVec, Glyph, ScaleFont},
    anyhow::{Context, Result},
    image::Rgba,
    sim2d::{
        application::Application,
        graphics::{AssetLoader, Image},
        math::Vec2,
        Sim2D, Sketch,
    },
};

fn layout_text<F, SF>(font: SF, text: impl AsRef<str>) -> Vec<Glyph>
where
    F: Font,
    SF: ScaleFont<F>,
{
    let mut glyphs = vec![];
    let v_advance = (font.height() + font.line_gap()).round();

    let mut previous: Option<Glyph> = None;

    let mut caret = ab_glyph::point(0.0, font.ascent().round());
    for c in text.as_ref().chars() {
        if c.is_control() {
            if c == '\n' {
                caret = ab_glyph::point(0.0, (caret.y + v_advance).round());
            }
            continue;
        }

        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = previous.take() {
            caret.x += font.kern(previous.id, glyph.id);
            caret.x = caret.x.round();
        }
        glyph.position = caret;

        previous = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id).round();

        glyphs.push(glyph);
    }

    glyphs
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
        let scaled_font = font.as_scaled(256.0);

        let glyphs = layout_text(scaled_font, "Hello World\naoue");
        let outlines = glyphs
            .iter()
            .filter_map(|glyph| scaled_font.outline_glyph(glyph.clone()))
            .collect::<Vec<_>>();

        let h = outlines
            .iter()
            .map(|outline| outline.px_bounds().max.y.ceil() as u32)
            .max()
            .unwrap();

        let min_x = outlines
            .iter()
            .map(|outline| outline.px_bounds().min.x.ceil() as u32)
            .min()
            .unwrap();

        let max_x = outlines
            .iter()
            .map(|outline| outline.px_bounds().max.x.ceil() as u32)
            .max()
            .unwrap();

        let w = max_x - min_x;

        let mut img = image::DynamicImage::new_rgba8(w, h).to_rgba8();
        img.fill(0);

        log::info!("min_x - max_x = {} - {} = {}", min_x, max_x, w);
        log::info!("first {:?}", outlines.first().unwrap().px_bounds());
        log::info!("last {:?}", outlines.last().unwrap().px_bounds());

        for outline in outlines {
            let bounds = outline.px_bounds();
            let left = bounds.min.x.ceil() as u32 - min_x;
            outline.draw(|px, py, v| {
                let x = px + left;
                let y = py + bounds.min.y as u32 - 1;
                let p = img.get_pixel_mut(x, y);
                *p = image::Rgba::from([
                    255, 255, 255,
                    255,
                    //p.0[3].saturating_add((v * 255.0).round() as u8),
                ]);
            });
        }

        self.font_atlas = _loader.load_image(img, true);

        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.image = self.font_atlas;
        sim.g.rect_centered(
            Vec2::new(0.0, 0.0),
            Vec2::new(self.font_atlas.width(), self.font_atlas.height()),
            //Vec2::new(200.0, 200.0),
            0.0,
        );
    }
}

fn main() -> Result<()> {
    Application::run(TextRendering::default())
}
