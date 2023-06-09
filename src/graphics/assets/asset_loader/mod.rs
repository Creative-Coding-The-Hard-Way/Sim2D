mod new_assets;

use {
    super::FontId,
    crate::graphics::{
        assets::{CachedFont, Image, TextureId},
        vulkan_api::RenderDevice,
        GraphicsError,
    },
    ::image::RgbaImage,
    ab_glyph::{Font, FontVec, PxScaleFont},
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

    font_base_index: usize,
    fonts: Vec<Arc<CachedFont>>,
    cached_fonts: HashMap<String, FontId>,

    render_device: Arc<RenderDevice>,
}

impl AssetLoader {
    pub fn load_font(
        &mut self,
        font: PxScaleFont<FontVec>,
        name: impl AsRef<str>,
    ) -> Result<FontId, GraphicsError> {
        let cache_id = name.as_ref().to_owned();
        if let Some(font_id) = self.cached_fonts.get(&cache_id) {
            return Ok(*font_id);
        }

        let cached_alphabet = "
            abcdefghijklmnopqrstuvwxyz
            ABCDEFGHIJKLMNOPQRSTUVWXYZ
            1234567890{}()[]*&^%$#@!+=
            -/\\\"'`;:<>.,_|
            ";
        let (atlas, glyph_uvs) =
            CachedFont::build_atlas(&font, cached_alphabet);

        let atlas_image = self.load_image(atlas, true, &cache_id);

        let font = Arc::new(CachedFont::new(atlas_image, font, glyph_uvs));
        let font_id = FontId::from_raw(self.font_base_index + self.fonts.len());
        self.fonts.push(font);

        self.cached_fonts.insert(cache_id, font_id);

        Ok(font_id)
    }

    pub fn load_font_file(
        &mut self,
        file_path: impl AsRef<Path>,
        size: f32,
    ) -> Result<FontId, GraphicsError> {
        let cache_id: String =
            format!("{}-{}", file_path.as_ref().to_str().unwrap(), size);
        if let Some(font_id) = self.cached_fonts.get(&cache_id) {
            return Ok(*font_id);
        }

        let font_data =
            std::fs::read(file_path).context("Unable to read font!")?;
        let scaled_font = FontVec::try_from_vec(font_data)
            .context("unable to create a font!")?
            .into_scaled(size);

        self.load_font(scaled_font, cache_id)
    }

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
        font_base_index: usize,
        cached_fonts: HashMap<String, FontId>,
    ) -> Self {
        Self {
            texture_base_index,
            texture_sources: vec![],
            cached_textures,

            font_base_index,
            cached_fonts,
            fonts: vec![],

            render_device,
        }
    }

    pub(crate) fn texture_base_index(&self) -> usize {
        self.texture_base_index
    }

    pub(crate) fn font_base_index(&self) -> usize {
        self.font_base_index
    }

    pub(crate) fn cached_textures(&self) -> &HashMap<String, Image> {
        &self.cached_textures
    }

    pub(crate) fn cached_fonts(&self) -> &HashMap<String, FontId> {
        &self.cached_fonts
    }

    pub(crate) fn fonts(&self) -> &[Arc<CachedFont>] {
        &self.fonts
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
