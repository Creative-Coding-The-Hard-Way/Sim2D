mod glfw_api;

use crate::math::Vec2;

/// Represents the Window's state.
///
/// Sketches can modify the state to change properties about the window.
/// Notably, the window size can be controlled with the WindowState.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowState {
    // Window Size and fullscreen variables
    toggle_fullscreen: bool,
    is_fullscreen: bool,
    needs_resized: bool,

    // Set to true when the window should be closed.
    should_close: bool,

    // Track window width and position for fullscreen toggling.
    windowed_width: i32,
    windowed_height: i32,
    window_x: i32,
    window_y: i32,

    // The current width and height (in pixels) of the window.
    width: f32,
    height: f32,

    // Input state variables
    mouse_pos: Vec2,
}

// Public API
// ----------

impl WindowState {
    pub fn set_should_close(&mut self, should_close: bool) {
        self.should_close = should_close;
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.needs_resized = true;
    }

    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn toggle_fullscreen(&mut self) {
        self.toggle_fullscreen = true;
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}
