use {
    super::{super::texture::TextureLoader, AssetLoader, Source},
    crate::graphics::{vulkan_api::Texture2D, GraphicsError},
    ash::vk,
    std::sync::Arc,
};

/// Represents new assets to include in the atlas.
pub struct NewAssetsCommand {
    pub base_index: usize,
    pub textures: Vec<Arc<Texture2D>>,
    pub image_acquire_barriers: Vec<vk::ImageMemoryBarrier2>,
}

/// # Safety
///
/// It is safe to SEND NewAssetsCommands despite owning vk::ImageMemoryBarrier2
/// because the next pointer is not used.
unsafe impl Send for NewAssetsCommand {}

// Private API
// -----------

impl NewAssetsCommand {
    pub(crate) fn new(
        asset_loader: AssetLoader,
    ) -> Result<Self, GraphicsError> {
        let mut loader =
            unsafe { TextureLoader::new(asset_loader.render_device.clone())? };

        for source in asset_loader.texture_sources {
            match &source {
                Source::FilePath(path) => unsafe {
                    loader.load_texture_2d_from_file(path)?
                },
                Source::Image(ref img) => unsafe {
                    loader.load_texture_2d_from_image(img.clone())
                },
            };
        }

        let (textures, image_acquire_barriers) =
            unsafe { loader.load_textures()? };

        Ok(NewAssetsCommand {
            base_index: asset_loader.base_index,
            textures,
            image_acquire_barriers,
        })
    }
}
