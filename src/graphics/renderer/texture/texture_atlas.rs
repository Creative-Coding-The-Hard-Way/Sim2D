use {
    crate::graphics::{
        vulkan_api::{RenderDevice, Texture2D},
        AssetLoader, GraphicsError, NewAssetsCommand,
    },
    std::sync::Arc,
};

/// A collection of all available textures for this application.
pub struct TextureAtlas {
    asset_loader: Option<AssetLoader>,
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
            asset_loader: Some(AssetLoader::new(render_device.clone(), 0)),
            textures: vec![],
            render_device,
        })
    }

    /// Take the asset loader instance to use for asynchronously loading assets.
    pub fn take_asset_loader(&mut self) -> AssetLoader {
        self.asset_loader.take().unwrap()
    }

    /// Add new textures to the atlas based on the new assets command.
    pub fn load_assets(&mut self, new_assets_cmd: &NewAssetsCommand) {
        debug_assert!(self.textures.len() == new_assets_cmd.base_index);
        self.textures.extend_from_slice(&new_assets_cmd.textures);
        self.asset_loader = Some(AssetLoader::new(
            self.render_device.clone(),
            self.textures.len(),
        ));
    }

    pub fn textures(&self) -> &[Arc<Texture2D>] {
        &self.textures
    }
}
