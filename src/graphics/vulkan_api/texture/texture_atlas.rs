use {
    crate::graphics::{
        vulkan_api::{RenderDevice, Texture2D, TextureLoader},
        GraphicsError,
    },
    std::{
        path::{Path, PathBuf},
        sync::Arc,
    },
};

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct TextureId {
    index: i32,
}

impl Default for TextureId {
    fn default() -> Self {
        Self::no_texture()
    }
}

impl TextureId {
    pub fn no_texture() -> Self {
        Self { index: -1 }
    }

    pub(crate) fn from_raw(index: i32) -> Self {
        Self { index }
    }

    pub(crate) fn get_index(&self) -> i32 {
        self.index
    }
}

enum Source {
    FilePath(PathBuf),
    Image(image::RgbaImage),
}

/// A collection of all available textures for this application.
#[derive(Default)]
pub struct TextureAtlas {
    texture_reservations: Vec<Source>,
}

impl TextureAtlas {
    pub fn load_file(&mut self, file_path: impl AsRef<Path>) -> TextureId {
        let source = Source::FilePath(file_path.as_ref().to_owned());
        self.load_texture_2d_from_source(source)
    }

    pub fn load_image(&mut self, img: image::RgbaImage) -> TextureId {
        self.load_texture_2d_from_source(Source::Image(img))
    }
}

// Private API
// -----------

impl TextureAtlas {
    fn load_texture_2d_from_source(&mut self, source: Source) -> TextureId {
        let index = self.texture_reservations.len();
        self.texture_reservations.push(source);
        TextureId::from_raw(index as i32)
    }

    pub(crate) fn load_all_textures(
        self,
        render_device: Arc<RenderDevice>,
    ) -> Result<Vec<Arc<Texture2D>>, GraphicsError> {
        let mut loader = unsafe { TextureLoader::new(render_device)? };

        let mut textures = Vec::with_capacity(self.texture_reservations.len());

        for source in self.texture_reservations.iter() {
            let texture = match source {
                Source::FilePath(path) => unsafe {
                    loader.load_texture_2d_from_file(path)?
                },
                Source::Image(ref img) => unsafe {
                    loader.load_texture_2d_from_image(img)?
                },
            };
            textures.push(Arc::new(texture));
        }

        Ok(textures)
    }
}
