mod asset_loader;
mod image;
mod text;

use {
    crate::graphics::vulkan_api::{RenderDevice, Texture2D},
    ash::vk,
    std::{collections::HashMap, sync::Arc},
};

pub use self::{
    asset_loader::{AssetLoader, NewAssets, TextureSource},
    image::Image,
    text::CachedFont,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TextureId {
    index: i32,
}

impl TextureId {
    pub const fn no_texture() -> Self {
        Self { index: -1 }
    }
}

impl TextureId {
    fn from_raw(index: usize) -> Self {
        Self {
            index: index as i32,
        }
    }

    pub(crate) fn raw(&self) -> i32 {
        self.index
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct FontId {
    index: usize,
}

impl FontId {
    pub const fn default_font() -> Self {
        Self { index: 0 }
    }
}

impl FontId {
    fn from_raw(index: usize) -> Self {
        Self { index }
    }

    pub(crate) fn raw(&self) -> usize {
        self.index
    }
}

/// All assets available for use by the renderer.
pub struct Assets {
    textures: Vec<Arc<Texture2D>>,
    cached_textures: HashMap<String, Image>,

    fonts: Vec<Arc<CachedFont>>,
    cached_fonts: HashMap<String, FontId>,

    loader: Option<AssetLoader>,
    render_device: Arc<RenderDevice>,
}

impl Assets {
    pub fn new(render_device: Arc<RenderDevice>) -> Self {
        Self {
            textures: vec![],
            cached_textures: HashMap::default(),

            fonts: vec![],
            cached_fonts: HashMap::default(),

            loader: Some(AssetLoader::new(
                render_device.clone(),
                0,
                HashMap::default(),
                0,
                HashMap::default(),
            )),
            render_device,
        }
    }

    pub fn take_asset_loader(&mut self) -> AssetLoader {
        self.loader.take().unwrap()
    }

    pub fn new_assets(
        &mut self,
        new_assets: NewAssets,
    ) -> Vec<vk::ImageMemoryBarrier2> {
        assert!(
            self.textures.len() == new_assets.asset_loader.texture_base_index()
        );
        assert!(self.fonts.len() == new_assets.asset_loader.font_base_index());

        self.textures.extend(new_assets.textures.into_iter());
        self.cached_textures.extend(
            new_assets
                .asset_loader
                .cached_textures()
                .iter()
                .map(|(k, v)| (k.clone(), *v)),
        );

        self.fonts
            .extend_from_slice(new_assets.asset_loader.fonts());
        self.cached_fonts.extend(
            new_assets
                .asset_loader
                .cached_fonts()
                .iter()
                .map(|(k, v)| (k.clone(), *v)),
        );

        self.loader = Some(AssetLoader::new(
            self.render_device.clone(),
            self.textures.len(),
            self.cached_textures.clone(),
            self.fonts.len(),
            self.cached_fonts.clone(),
        ));

        log::trace!("Loaded assets: {:#?}", self.cached_textures);

        new_assets.image_acquire_barriers
    }

    pub fn textures(&self) -> &[Arc<Texture2D>] {
        &self.textures
    }

    pub fn fonts(&self) -> &[Arc<CachedFont>] {
        &self.fonts
    }
}
