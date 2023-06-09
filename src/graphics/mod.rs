mod assets;
mod error;
mod renderer;
pub(crate) mod vulkan_api;

use {crate::math::Vec2, std::sync::Arc, vulkan_api::SpriteData};

pub(crate) use self::assets::NewAssets;
pub use self::{
    assets::{AssetLoader, Assets, CachedFont, FontId, Image, TextureId},
    error::GraphicsError,
    renderer::Renderer,
};

pub struct G2D {
    cached_fonts: Vec<Arc<CachedFont>>,
    sprites: Vec<SpriteData>,

    pub font: FontId,
    pub clear_color: [f32; 4],
    pub fill_color: [f32; 4],
    pub image: Image,
    pub line_width: f32,
}

impl G2D {
    pub fn new(assets: &Assets) -> Self {
        Self {
            font: FontId::default_font(),
            cached_fonts: assets.fonts().to_vec(),
            sprites: Vec::with_capacity(10_000),
            clear_color: [1.0, 1.0, 1.0, 1.0],
            fill_color: [1.0, 1.0, 1.0, 1.0],
            image: Image::none(),
            line_width: 1.0,
        }
    }

    pub fn rect_centered(&mut self, pos: Vec2, size: Vec2, angle: f32) {
        self.sprites.push(SpriteData {
            pos: [pos.x, pos.y],
            size: [size.x, size.y],
            rgba: self.fill_color,
            tex: self.image.texture_id().raw() as f32,
            angle,
            uv_scale: [1.0, 1.0],
            uv_offset: [0.0, 0.0],
            ..Default::default()
        });
    }

    pub fn rect(&mut self, top_left: Vec2, size: Vec2, angle: f32) {
        self.sprites.push(SpriteData {
            pos: [top_left.x, top_left.y],
            size: [size.x, size.y],
            rgba: self.fill_color,
            tex: self.image.texture_id().raw() as f32,
            angle,
            center_offset: [0.5, -0.5],
            ..Default::default()
        });
    }

    pub fn rect_uvs(
        &mut self,
        top_left: Vec2,
        size: Vec2,
        uv_top_left: Vec2,
        uv_scale: Vec2,
    ) {
        self.sprites.push(SpriteData {
            pos: [top_left.x, top_left.y],
            size: [size.x, size.y],
            rgba: self.fill_color,
            tex: self.image.texture_id().raw() as f32,
            angle: 0.0,
            center_offset: [0.5, -0.5],
            uv_offset: uv_top_left.into(),
            uv_scale: uv_scale.into(),
        });
    }

    pub fn line(&mut self, start: Vec2, end: Vec2) {
        let d = end - start;
        let len = d.magnitude();
        let midpoint = start + 0.5 * d;
        let angle =
            ((d.y / len) / (d.x / len)).atan() + std::f32::consts::FRAC_PI_2;
        self.rect_centered(midpoint, Vec2::new(self.line_width, len), angle);
    }

    pub fn text(&mut self, pos: Vec2, text: impl AsRef<str>) {
        let font = &self.cached_fonts[self.font.raw()];

        let (glyph_sprites, _, _) = font.layout_paragraph_geometry(text);

        let original_image = self.image;
        self.image = font.atlas;

        for sprite in &glyph_sprites {
            self.rect_uvs(
                sprite.top_left + pos,
                sprite.size,
                sprite.uv_top_left,
                sprite.uv_size,
            )
        }

        self.image = original_image;
    }
}

// Private API
// -----------

impl G2D {
    fn get_sprites(&self) -> &[SpriteData] {
        &self.sprites
    }

    fn reset(&mut self) {
        self.sprites.clear();
    }
}
