use crate::graphics::TextureId;

/// An Image is usable as a Texture by Sim2d Sketches.
#[derive(Copy, Clone, Debug)]
pub struct Image {
    texture_id: TextureId,
    width: f32,
    height: f32,
}

// Public API
// ----------

impl Image {
    /// A constant representing the absence of an Image.
    pub const fn none() -> Self {
        Self {
            texture_id: TextureId::no_texture(),
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn texture_id(&self) -> TextureId {
        self.texture_id
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::none()
    }
}

// Private API
// -----------

impl Image {
    pub(crate) fn new(texture_id: TextureId, width: f32, height: f32) -> Self {
        Self {
            texture_id,
            width,
            height,
        }
    }
}
