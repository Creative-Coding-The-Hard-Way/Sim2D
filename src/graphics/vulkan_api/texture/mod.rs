mod texture_atlas;
mod texture_id;
mod texture_loader;

pub use self::{
    texture_atlas::TextureAtlas, texture_id::TextureId,
    texture_loader::TextureLoader,
};
use crate::graphics::vulkan_api::raii;

/// Represents a 2D rgba texture which can be used by shaders.
#[derive(Debug)]
pub struct Texture2D {
    pub image_view: raii::ImageView,
    pub image: raii::Image,
}
