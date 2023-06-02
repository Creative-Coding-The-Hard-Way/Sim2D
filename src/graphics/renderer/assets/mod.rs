mod image;
mod new_assets_cmd;

use {
    crate::graphics::{
        renderer::texture::TextureId, vulkan_api::RenderDevice, GraphicsError,
    },
    ::image::RgbaImage,
    anyhow::Context,
    std::{path::Path, sync::Arc},
};

pub use self::{image::Image, new_assets_cmd::NewAssetsCommand};

#[derive(Debug, Clone)]
pub struct TextureSource {
    img: RgbaImage,
    generate_mipmaps: bool,
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
        generate_mipmaps: bool,
    ) -> Result<Image, GraphicsError> {
        let img = Self::load_image_from_file(file_path)?;
        Ok(self.load_image(img, generate_mipmaps))
    }

    pub fn load_image(
        &mut self,
        img: RgbaImage,
        generate_mipmaps: bool,
    ) -> Image {
        let index = self.base_index + self.texture_sources.len();
        let width = img.width() as f32;
        let height = img.height() as f32;
        let source = TextureSource {
            img,
            generate_mipmaps,
        };
        self.texture_sources.push(source);
        Image::new(TextureId::from_raw(index as i32), width, height)
    }
}

impl AssetLoader {
    fn load_image_from_file(
        texture_path: impl AsRef<Path>,
    ) -> Result<RgbaImage, GraphicsError> {
        let img = ::image::io::Reader::open(&texture_path)
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
