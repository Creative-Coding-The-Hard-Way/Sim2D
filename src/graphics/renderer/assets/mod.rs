use {
    super::texture::TextureLoader,
    crate::graphics::{
        renderer::texture::TextureId,
        vulkan_api::{RenderDevice, Texture2D},
        GraphicsError,
    },
    ash::vk,
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
    pub image_acquire_barriers: Vec<vk::ImageMemoryBarrier2>,
}

/// # Safety
///
/// It is safe to SEND NewAssetsCommands despite owning vk::ImageMemoryBarrier2
/// because the next pointer is not used.
unsafe impl Send for NewAssetsCommand {}

impl NewAssetsCommand {
    pub fn new(base_index: usize) -> Self {
        Self {
            base_index,
            textures: Vec::default(),
            image_acquire_barriers: Vec::default(),
        }
    }
}

// Private API
// -----------

impl AssetLoader {
    pub(crate) fn build_new_assets_command(
        self,
    ) -> Result<NewAssetsCommand, GraphicsError> {
        let mut new_assets_cmd = NewAssetsCommand::new(self.base_index);

        let mut loader =
            unsafe { TextureLoader::new(self.render_device.clone())? };

        for source in self.texture_sources {
            match &source {
                Source::FilePath(path) => unsafe {
                    loader.load_texture_2d_from_file(path)?
                },
                Source::Image(ref img) => unsafe {
                    loader.load_texture_2d_from_image(img.clone())
                },
            };
        }

        let (textures, grahpics_acquire_barriers) =
            unsafe { loader.load_textures()? };

        new_assets_cmd.textures = textures;
        new_assets_cmd.image_acquire_barriers = grahpics_acquire_barriers;

        Ok(new_assets_cmd)
    }
}
