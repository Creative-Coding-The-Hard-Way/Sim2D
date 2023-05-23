use {
    crate::{graphics::vulkan_api::TextureAtlas, sim2d::Sim2D},
    anyhow::Result,
};

/// Sketch is created after the GLFW window is created, but is allowed to
/// configure the window for things like resizability and event polling.
pub trait Sketch {
    /// Create a new instance of this sketch.
    fn new(sim: &mut Sim2D) -> Result<Self>
    where
        Self: Sized;

    /// Load any textures needed by the sketch.
    ///
    /// # Params
    ///
    /// * `texture_atlas` - The texture atlas can be used to create / load
    ///   textures from disk and keep their texture ids.
    fn preload(&mut self, _texture_atlas: &mut TextureAtlas) -> Result<()> {
        Ok(())
    }

    /// Called any time the mouse is moved on screen.
    ///
    /// Sim2D retains all information regarding the mouse's position and
    /// movement.
    fn mouse_moved(&mut self, _sim: &mut Sim2D) -> Result<()> {
        Ok(())
    }

    /// Called any time the mouse is pressed.
    ///
    /// Sim2D retains all information regarding the mouse's position and
    /// movement.
    fn mouse_pressed(&mut self, _sim: &mut Sim2D) -> Result<()> {
        Ok(())
    }

    /// Called any time the mouse is released.
    fn mouse_released(&mut self, _sim: &mut Sim2D) -> Result<()> {
        Ok(())
    }

    /// Called once per frame.
    fn update(&mut self, _sim: &mut Sim2D) -> Result<()> {
        Ok(())
    }
}
