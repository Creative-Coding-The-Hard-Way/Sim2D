mod new_assets_cmd;

use {
    crate::graphics::{renderer::texture::TextureId, vulkan_api::RenderDevice},
    std::{
        path::{Path, PathBuf},
        sync::Arc,
    },
};

pub use self::new_assets_cmd::NewAssetsCommand;

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
