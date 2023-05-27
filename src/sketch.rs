use crate::{graphics::TextureAtlas, sim2d::Sim2D};

/// A sketch is the primary entrypoint for the application.
pub trait Sketch {
    /// Load any textures needed by the sketch.
    ///
    /// # Params
    ///
    /// * `texture_atlas` - The texture atlas can be used to create / load
    ///   textures from disk and keep their texture ids.
    fn preload(&mut self, _texture_atlas: &mut TextureAtlas) {}

    /// Setup the sketch. This method is called one time after the call to
    /// preload.
    fn setup(&mut self, _sim: &mut Sim2D) {}

    /// Called any time the mouse is moved on screen.
    ///
    /// Sim2D retains all information regarding the mouse's position and
    /// movement.
    fn mouse_moved(&mut self, _sim: &mut Sim2D) {}

    /// Called any time the mouse is pressed.
    ///
    /// Sim2D retains all information regarding the mouse's position and
    /// movement.
    fn mouse_pressed(&mut self, _sim: &mut Sim2D) {}

    /// Called any time the mouse is released.
    fn mouse_released(&mut self, _sim: &mut Sim2D) {}

    /// Called when a key on the keyboard is pressed.
    fn key_pressed(&mut self, _sim: &mut Sim2D, _key: glfw::Key) {}

    /// Called when a key on the keyboard is released.
    fn key_released(&mut self, _sim: &mut Sim2D, _key: glfw::Key) {}

    /// Called once per frame.
    fn update(&mut self, _sim: &mut Sim2D);
}
