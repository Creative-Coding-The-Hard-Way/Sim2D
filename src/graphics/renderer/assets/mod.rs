mod new_assets_cmd;

use {
    crate::graphics::{
        renderer::texture::TextureId, vulkan_api::RenderDevice, GraphicsError,
    },
    anyhow::Context,
    std::{path::Path, sync::Arc},
};

pub use self::new_assets_cmd::NewAssetsCommand;

#[derive(Debug, Clone)]
pub struct TextureSource {
    img: image::RgbaImage,
}

/// The public API for loading new images and textures for use in sketches.
#[derive(Debug)]
pub struct AssetLoader {
    base_index: usize,
    texture_sources: Vec<TextureSource>,
    render_device: Arc<RenderDevice>,
}

impl AssetLoader {
    pub(crate) fn new(
        render_device: Arc<RenderDevice>,
        base_index: usize,
    ) -> Self {
        Self {
            base_index,
            texture_sources: vec![],
            render_device,
        }
    }

    pub fn load_file(
        &mut self,
        file_path: impl AsRef<Path>,
    ) -> Result<TextureId, GraphicsError> {
        let img = Self::load_image_from_file(file_path)?;
        Ok(self.load_image(img))
    }

    pub fn load_image(&mut self, img: image::RgbaImage) -> TextureId {
        let index = self.base_index + self.texture_sources.len();
        let source = TextureSource { img };
        self.texture_sources.push(source);
        TextureId::from_raw(index as i32)
    }
}

impl AssetLoader {
    fn load_image_from_file(
        texture_path: impl AsRef<Path>,
    ) -> Result<image::RgbaImage, GraphicsError> {
        let img = image::io::Reader::open(&texture_path)
            .with_context(|| {
                format!(
                    "Unable to read texture image from path {:?}",
                    texture_path.as_ref()
                )
            })?
            .decode()
            .with_context(|| {
                format!(
                    "Unable to decode texture image at {:?}",
                    texture_path.as_ref()
                )
            })?
            .into_rgba8();
        Ok(img)
    }
}
