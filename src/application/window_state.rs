use {
    crate::math::{vec2, Vec2f, Vec2ui},
    glfw::Action,
    std::collections::BTreeSet,
};

pub enum WindowEvent {
    FramebufferResized,
    WindowResized,
    MouseMoved,
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    fn from_glfw(button: &glfw::MouseButton) -> Option<Self> {
        match *button {
            glfw::MouseButton::Button1 => Some(Self::Left),
            glfw::MouseButton::Button2 => Some(Self::Right),
            glfw::MouseButton::Button3 => Some(Self::Middle),
            _ => None,
        }
    }
}

/// The current Window state.
///
/// Values are updated automatically for the application to inspect.
pub struct WindowState {
    framebuffer_size: Vec2ui,
    size: Vec2f,
    mouse_pixels: Vec2f,
    mouse: Vec2f,
    mouse_buttons: BTreeSet<MouseButton>,
}

impl WindowState {
    pub(super) fn new(window: &glfw::Window) -> Self {
        let (fb_w, fb_h) = window.get_framebuffer_size();
        let (w, h) = window.get_size();
        let (m_x, m_y) = window.get_cursor_pos();

        let size = vec2(w as f32, h as f32);
        let mouse_pixels = vec2(m_x as f32, m_y as f32);
        let mouse = mouse_pixels.component_div(&size);

        Self {
            framebuffer_size: vec2(fb_w as u32, fb_h as u32),
            size,
            mouse_pixels,
            mouse,
            mouse_buttons: BTreeSet::new(),
        }
    }

    pub(super) fn handle_event(
        &mut self,
        event: &glfw::WindowEvent,
    ) -> Option<WindowEvent> {
        match &event {
            glfw::WindowEvent::FramebufferSize(w, h) => {
                self.framebuffer_size = vec2(*w as u32, *h as u32);
                Some(WindowEvent::FramebufferResized)
            }
            glfw::WindowEvent::Size(w, h) => {
                self.size = vec2(*w as f32, *h as f32);
                Some(WindowEvent::WindowResized)
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                self.mouse_pixels = vec2(*x as f32, *y as f32);
                let m = self.mouse_pixels.component_div(&self.size);
                self.mouse = vec2(m.x - 0.5, 0.5 - m.y);
                Some(WindowEvent::MouseMoved)
            }
            glfw::WindowEvent::MouseButton(glfw_button, Action::Press, _) => {
                MouseButton::from_glfw(glfw_button).map(|btn| {
                    self.mouse_buttons.insert(btn);
                    WindowEvent::MouseButtonPressed(btn)
                })
            }
            glfw::WindowEvent::MouseButton(glfw_button, Action::Release, _) => {
                MouseButton::from_glfw(glfw_button).map(|btn| {
                    self.mouse_buttons.remove(&btn);
                    WindowEvent::MouseButtonReleased(btn)
                })
            }
            _ => None,
        }
    }
}

// Public API

impl WindowState {
    // Mouse API
    // ------------------------------------------------------------------------

    /// Get the current mouse position in pixels. (0, 0) is the top left of the
    /// screen, while (screen_width, screen_height) is the bottom right of
    /// the screen.
    pub fn mouse_pixels(&self) -> &Vec2f {
        &self.mouse_pixels
    }

    /// Get the normalized mouse position. (-0.5, 0.5) is the top left of the
    /// screen while (0.5, -0.5) is the bottom right of the screen.
    pub fn mouse(&self) -> &Vec2f {
        &self.mouse
    }

    /// Returns true when the given button is pressed.
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    /// Returns true when ANY mouse button pressed.
    pub fn any_button_pressed(&self) -> bool {
        !self.mouse_buttons.is_empty()
    }

    // Window Size API
    // ------------------------------------------------------------------------

    /// Get the current size of the window.
    ///
    /// Note that this can differ from the framebuffer size depending on the
    /// system and the content scaling. This size can be used to compare
    /// with the mouse position.
    pub fn size(&self) -> &Vec2f {
        &self.size
    }

    /// Get the current framebuffer size. This is useful for computing the
    /// swapchain extent or for creating pixel-perfect uniform
    /// transformations.
    pub fn framebuffer_size(&self) -> &Vec2ui {
        &self.framebuffer_size
    }
}
