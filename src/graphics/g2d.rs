use {
    super::vulkan_api::SpriteData,
    crate::{graphics::vulkan_api::TextureId, math::Vec2},
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ShapeVertex {
    pub pos: Vec2,
    pub uv: Vec2,
}

pub struct G2D {
    sprites: Vec<SpriteData>,

    pub clear_color: [f32; 4],
    pub fill_color: [f32; 4],
    pub texture: TextureId,
    pub line_width: f32,
}

impl Default for G2D {
    fn default() -> Self {
        Self {
            sprites: Vec::with_capacity(10_000),
            clear_color: [1.0, 1.0, 1.0, 1.0],
            fill_color: [1.0, 1.0, 1.0, 1.0],
            texture: TextureId::no_texture(),
            line_width: 1.0,
        }
    }
}

impl G2D {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn rect_centered(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.sprites.push(SpriteData {
            pos: [x, y],
            size: [width, height],
            rgba: self.fill_color,
            tex: self.texture.get_index(),
            ..Default::default()
        });
    }
}

// Private API
// -----------

impl G2D {
    pub(crate) fn get_sprites(&self) -> &[SpriteData] {
        &self.sprites
    }

    pub(crate) fn reset(&mut self) {
        self.sprites.clear();
    }
}
