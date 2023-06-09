mod new_assets;

use {
    crate::graphics::{
        renderer::{Image, TextureId},
        vulkan_api::RenderDevice,
        GraphicsError,
    },
    ::image::RgbaImage,
    anyhow::Context,
    std::{collections::HashMap, path::Path, sync::Arc},
};

pub use self::new_assets::NewAssets;

#[derive(Debug, Clone)]
pub struct TextureSource {
    img: RgbaImage,
    generate_mipmaps: bool,
}

/// The public API for loading new images and textures for use in sketches.
pub struct AssetLoader {
    texture_base_index: usize,
    texture_sources: Vec<TextureSource>,
    cached_textures: HashMap<String, Image>,

    render_device: Arc<RenderDevice>,
}

impl AssetLoader {
    pub fn load_image_file(
        &mut self,
        file_path: impl AsRef<Path>,
        generate_mipmaps: bool,
    ) -> Result<Image, GraphicsError> {
        let cache_id: String = file_path.as_ref().to_str().unwrap().to_owned();

        if let Some(image) = self.cached_textures.get(&cache_id) {
            return Ok(*image);
        }

        let img = Self::load_image_from_file(file_path)?;
        Ok(self.load_image(img, generate_mipmaps, cache_id))
    }

    pub fn load_image(
        &mut self,
        img: RgbaImage,
        generate_mipmaps: bool,
        name: impl AsRef<str>,
    ) -> Image {
        if let Some(image) = self.cached_textures.get(name.as_ref()) {
            return *image;
        }

        let index = self.texture_base_index + self.texture_sources.len();
        let width = img.width() as f32;
        let height = img.height() as f32;
        let source = TextureSource {
            img,
            generate_mipmaps,
        };
        self.texture_sources.push(source);

        let image = Image::new(TextureId::from_raw(index), width, height);
        self.cached_textures.insert(name.as_ref().to_owned(), image);
        image
    }
}

impl AssetLoader {
    pub(crate) fn new(
        render_device: Arc<RenderDevice>,
        texture_base_index: usize,
        cached_textures: HashMap<String, Image>,
    ) -> Self {
        Self {
            texture_base_index,
            texture_sources: vec![],
            cached_textures,
            render_device,
        }
    }

    pub(crate) fn texture_base_index(&self) -> usize {
        self.texture_base_index
    }

    pub(crate) fn cached_textures(&self) -> &HashMap<String, Image> {
        &self.cached_textures
    }

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
