use {
    crate::graphics::{
        renderer::texture::{TextureId, TextureLoader},
        vulkan_api::{RenderDevice, Texture2D},
        GraphicsError,
    },
    std::{
        path::{Path, PathBuf},
        sync::Arc,
    },
};

#[derive(Debug, Clone)]
enum Source {
    FilePath(PathBuf),
    Image(image::RgbaImage),
}

/// The public API for loading new images and textures for use in sketches.
#[derive(Debug)]
pub struct AssetLoader {
    base_index: usize,
    texture_sources: Vec<Source>,
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

    pub fn load_file(&mut self, file_path: impl AsRef<Path>) -> TextureId {
        let index = self.base_index + self.texture_sources.len();
        let source = Source::FilePath(file_path.as_ref().to_owned());
        self.texture_sources.push(source);
        TextureId::from_raw(index as i32)
    }

    pub fn load_image(&mut self, img: image::RgbaImage) -> TextureId {
        let index = self.base_index + self.texture_sources.len();
        let source = Source::Image(img);
        self.texture_sources.push(source);
        TextureId::from_raw(index as i32)
    }
}

/// Represents new assets to include in the atlas.
pub struct NewAssetsCommand {
    pub base_index: usize,
    pub textures: Vec<Arc<Texture2D>>,
}

impl NewAssetsCommand {
    pub fn new(base_index: usize) -> Self {
        Self {
            base_index,
            textures: Vec::default(),
        }
    }
}

impl AssetLoader {
    pub(crate) fn build_new_assets_command(
        self,
    ) -> Result<NewAssetsCommand, GraphicsError> {
        let mut new_assets_cmd = NewAssetsCommand::new(self.base_index);

        let mut loader =
            unsafe { TextureLoader::new(self.render_device.clone())? };
        for source in self.texture_sources {
            let texture = match &source {
                Source::FilePath(path) => unsafe {
                    loader.load_texture_2d_from_file(path)?
                },
                Source::Image(ref img) => unsafe {
                    loader.load_texture_2d_from_image(img)?
                },
            };
            new_assets_cmd.textures.push(Arc::new(texture));
        }

        Ok(new_assets_cmd)
    }
}

/// A collection of all available textures for this application.
pub struct TextureAtlas {
    textures: Vec<Arc<Texture2D>>,
    render_device: Arc<RenderDevice>,
}

impl TextureAtlas {
    /// Create a new texture atlas.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - The application must drop this resource before the render device.
    pub unsafe fn new(
        render_device: Arc<RenderDevice>,
    ) -> Result<Self, GraphicsError> {
        Ok(Self {
            textures: vec![],
            render_device,
        })
    }

    pub fn new_asset_loader(&self) -> AssetLoader {
        AssetLoader::new(self.render_device.clone(), self.textures.len())
    }

    /// Add new textures to the atlas based on the new assets command.
    pub fn add_textures(&mut self, new_assets_cmd: &NewAssetsCommand) {
        debug_assert!(self.textures.len() == new_assets_cmd.base_index);
        self.textures.extend_from_slice(&new_assets_cmd.textures);
    }

    pub fn textures(&self) -> &[Arc<Texture2D>] {
        &self.textures
    }
}
