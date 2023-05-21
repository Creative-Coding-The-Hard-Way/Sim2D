use {
    crate::{
        application::GlfwWindow,
        graphics::{g2d::G2D, vulkan_api::TextureAtlas},
    },
    anyhow::Result,
};

/// Sketch is created after the GLFW window is created, but is allowed to
/// configure the window for things like resizability and event polling.
pub trait Sketch {
    /// Create a new instance of this sketch.
    ///
    /// # Params
    ///
    /// * `window` - A fully constructed application window. The implementation
    ///   can use this handle to resize the window, apply GLFW window hints,
    ///   toggle fullscren, and construct a Vulkan instance which can present
    ///   surfaces to the window.
    /// * `g2d` - The 2d graphics state machine.
    fn new(window: &mut GlfwWindow, g2d: &mut G2D) -> Result<Self>
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

    /// Handle a GLFW event and update the application sketch.
    ///
    /// # Params
    ///
    /// * `window` - The fully constructed application window. The application
    ///   can exit by calling `set_should_close` on the window.
    /// * `window_event` - The event currently being processed by the window.
    fn handle_event(
        &mut self,
        _window: &mut GlfwWindow,
        _window_event: glfw::WindowEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// Called each time through the main application loop after all events
    /// have been processed.
    ///
    /// Update is not called while an application is paused while minimized.
    ///
    /// # Params
    ///
    /// * `g2d` - The 2D graphics state machine.
    fn update(&mut self, _g2d: &mut G2D) -> Result<()> {
        Ok(())
    }
}
