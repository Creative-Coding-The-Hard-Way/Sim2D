use crate::{graphics::AssetLoader, sim2d::Sim2D};

pub type DynSketch = Box<dyn Sketch + Send + 'static>;

/// A sketch is the primary entrypoint for the application.
pub trait Sketch {
    /// Load any textures needed by the sketch.
    ///
    /// # Params
    ///
    /// * `texture_atlas` - The texture atlas can be used to create / load
    ///   textures from disk and keep their texture ids.
    fn preload(&mut self, _asset_loader: &mut AssetLoader) {}

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

    /// Implement to return a new boxed sketch to hand off to another sketch.
    ///
    /// This way sketches can be chained together.
    fn load_sketch(&mut self) -> Option<DynSketch> {
        None
    }

    /// Called once per frame.
    fn update(&mut self, _sim: &mut Sim2D);
}
