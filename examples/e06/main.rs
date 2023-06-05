use {
    ab_glyph::{
        Font, FontVec, Glyph, GlyphId, OutlinedGlyph, PxScaleFont, ScaleFont,
    },
    anyhow::{Context, Result},
    sim2d::{
        application::Application,
        graphics::{AssetLoader, Image},
        math::Vec2,
        Sim2D, Sketch,
    },
    std::collections::HashMap,
};

pub struct GlyphUV {
    pub top_left: Vec2,
    pub scale: Vec2,
}

pub struct BitmapFont {
    atlas: Image,
    font: PxScaleFont<FontVec>,
    glyph_uvs: HashMap<GlyphId, GlyphUV>,
}

impl BitmapFont {
    /// Build a new bitmap font.
    pub fn new(
        loader: &mut AssetLoader,
        font: PxScaleFont<FontVec>,
        alphabet: impl AsRef<str>,
    ) -> Self {
        let glyphs = alphabet
            .as_ref()
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| font.scaled_glyph(c));

        let outlines = glyphs
            .filter_map(|glyph| font.outline_glyph(glyph))
            .collect::<Vec<_>>();

        let (offsets, w, h) = Self::layout_outlines_for_atlas(&outlines);

        let mut img = image::DynamicImage::new_rgba8(w, h).to_rgba8();
        img.fill(0);

        let mut glyph_uvs: HashMap<GlyphId, GlyphUV> = HashMap::new();
        for (outline, offset) in outlines.iter().zip(offsets) {
            let bounds = outline.px_bounds();
            glyph_uvs.insert(
                outline.glyph().id,
                GlyphUV {
                    top_left: offset
                        .component_div(&Vec2::new(w as f32, h as f32)),
                    scale: Vec2::new(
                        bounds.width().ceil() / w as f32,
                        bounds.height().ceil() / h as f32,
                    ),
                },
            );

            outline.draw(|px, py, v| {
                let x = px + offset.x as u32;
                let y = py + offset.y as u32;
                let p = img.get_pixel_mut(x, y);
                *p = image::Rgba::from([
                    255,
                    255,
                    255,
                    p.0[3].saturating_add((v * 255.0).round() as u8),
                ]);
            });
        }

        let atlas = loader.load_image(img, true);

        Self {
            atlas,
            font,
            glyph_uvs,
        }
    }

    pub fn draw_text(&self, sim: &mut Sim2D, pos: Vec2, text: impl AsRef<str>) {
        let glyphs = Self::layout_paragraph(&self.font, text);
        let outlines = glyphs
            .into_iter()
            .filter_map(|glyph| self.font.outline_glyph(glyph));

        let original_img = sim.g.image;
        sim.g.image = self.atlas;

        for outline in outlines {
            let uvs = &self.glyph_uvs[&outline.glyph().id];
            let bounds = outline.px_bounds();
            sim.g.rect_uvs(
                Vec2::new(
                    pos.x + bounds.min.x.ceil(),
                    pos.y - bounds.min.y.ceil(),
                ),
                Vec2::new(bounds.width(), bounds.height()),
                uvs.top_left,
                uvs.scale,
            );
        }

        sim.g.image = original_img;
    }

    fn layout_paragraph<F, SF>(font: &SF, text: impl AsRef<str>) -> Vec<Glyph>
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

    /// Layout a stream of outlined glyphs such that none overlap.
    ///
    /// # Returns
    ///
    /// A tuple of:
    ///   The set of top left Vec2 offsets for each glyph.
    ///   The width of the area containing all glyphs.
    ///   The height of the area containing all glyphs.
    fn layout_outlines_for_atlas(
        outlines: &[OutlinedGlyph],
    ) -> (Vec<Vec2>, u32, u32) {
        let width_limit = 2048.0;
        let mut max_width = 0;
        let mut max_height = 0;
        let h_pad = 10.0;
        let v_pad = 10.0;
        let mut cursor = Vec2::new(h_pad, v_pad);

        let mut top_left_offsets = Vec::with_capacity(outlines.len());

        for outline in outlines {
            let bounds = outline.px_bounds();

            if (cursor.x + bounds.width() + h_pad) >= width_limit {
                cursor = Vec2::new(h_pad, max_height as f32 + v_pad);
            }

            let top_left = cursor;
            top_left_offsets
                .push(Vec2::new(top_left.x.round(), top_left.y.round()));
            cursor.x += bounds.width() + h_pad;

            max_height =
                max_height.max((top_left.y + bounds.height()).round() as u32);
            max_width = max_width.max(cursor.x.round() as u32);
        }

        (top_left_offsets, max_width, max_height)
    }
}

/// A slow-loading sketch to demo the loading screen.
///
/// Clicking the mouse will respawn the sketch from scratch for loading.
#[derive(Default)]
struct TextRendering {
    bitmap_font: Option<BitmapFont>,
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
        let scaled_font = font.into_scaled(32.0);

        let cached_glyphs = "
            abcdefghijklmnopqrstuvwxyz
            ABCDEFGHIJKLMNOPQRSTUVWXYZ
            1234567890{}()[]*&^%$#@!+=
            -/\\\"'`;:<>.,_
            ";

        self.bitmap_font =
            Some(BitmapFont::new(loader, scaled_font, cached_glyphs));

        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];

        let msg = format!(
            "Frame Time: {}ms\nFPS: {}",
            (sim.dt() * 1000.0 * 10.0).ceil() / 10.0,
            (10.0 / sim.dt()).ceil() / 10.0
        );

        self.bitmap_font.as_ref().unwrap().draw_text(
            sim,
            Vec2::new(0.0, 0.0),
            msg,
        );
    }
}

fn main() -> Result<()> {
    Application::run(TextRendering::default())
}
