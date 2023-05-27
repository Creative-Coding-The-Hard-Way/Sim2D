use crate::graphics::vulkan_api::raii;

/// Represents a 2D rgba texture which can be used by shaders.
#[derive(Debug)]
pub struct Texture2D {
    pub image_view: raii::ImageView,
    pub image: raii::Image,
}
