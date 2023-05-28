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

#[derive(Debug, Clone)]
struct Reservation {
    source: Source,
    texture: Option<Arc<Texture2D>>,
}

/// A collection of all available textures for this application.
#[derive(Clone)]
pub struct TextureAtlas {
    texture_reservations: Vec<Reservation>,
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
            texture_reservations: vec![],
            render_device,
        })
    }

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
    pub(crate) fn all_textures(&self) -> Vec<Arc<Texture2D>> {
        self.texture_reservations
            .iter()
            .filter_map(|reservation| reservation.texture.clone())
            .collect()
    }

    pub(crate) fn load_all_textures(&mut self) -> Result<(), GraphicsError> {
        let mut loader =
            unsafe { TextureLoader::new(self.render_device.clone())? };
        for reservation in self
            .texture_reservations
            .iter_mut()
            .filter(|reservation| reservation.texture.is_none())
        {
            let texture = match &reservation.source {
                Source::FilePath(path) => unsafe {
                    loader.load_texture_2d_from_file(path)?
                },
                Source::Image(ref img) => unsafe {
                    loader.load_texture_2d_from_image(img)?
                },
            };
            reservation.texture = Some(Arc::new(texture));
        }
        Ok(())
    }

    fn load_texture_2d_from_source(&mut self, source: Source) -> TextureId {
        let index = self.texture_reservations.len();
        self.texture_reservations.push(Reservation {
            source,
            texture: None,
        });
        TextureId::from_raw(index as i32)
    }
}
