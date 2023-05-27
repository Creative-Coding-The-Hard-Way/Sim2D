use {
    crate::{
        math::Vec2,
        window::{glfw_window::GlfwWindow, WindowState},
    },
    anyhow::{Context, Result},
    glfw::{Action, MouseButton, WindowEvent, WindowMode},
    std::collections::HashSet,
};

impl GlfwWindow {
    /// Create a new state instance.
    pub(crate) fn new_window_state(self: &GlfwWindow) -> WindowState {
        let (window_x, window_y) = self.get_pos();
        let (w, h) = self.get_size();
        let (mouse_x, mouse_y) = self.get_cursor_pos();
        WindowState {
            toggle_fullscreen: false,
            is_fullscreen: self.is_glfw_window_fullscreen(),
            needs_resized: false,
            should_close: self.should_close(),

            windowed_width: w,
            windowed_height: h,
            window_x,
            window_y,

            width: w as f32,
            height: h as f32,

            mouse_pos: Vec2::new(mouse_x as f32, mouse_y as f32),
            left_button_pressed: false,
            middle_button_pressed: false,
            right_button_pressed: false,

            keyboard_button_pressed: false,
            pressed_keys: HashSet::with_capacity(26),
        }
    }

    /// Update a GLFW window to reflect the current state.
    pub(crate) fn update_window_to_match(
        &mut self,
        window_state: &mut WindowState,
    ) -> Result<()> {
        if window_state.toggle_fullscreen {
            window_state.toggle_fullscreen = false;
            self.toggle_glfw_fullscreen(window_state)?;
        }

        if window_state.needs_resized {
            window_state.needs_resized = false;
            self.set_size(
                window_state.width as i32,
                window_state.height as i32,
            );
        }

        self.set_should_close(window_state.should_close);
        Ok(())
    }

    pub(crate) fn handle_event(
        &mut self,
        window_state: &mut WindowState,
        window_event: &WindowEvent,
    ) -> Result<()> {
        match *window_event {
            WindowEvent::MouseButton(button, Action::Press, _) => {
                match button {
                    MouseButton::Button1 => {
                        window_state.left_button_pressed = true
                    }
                    MouseButton::Button2 => {
                        window_state.right_button_pressed = true
                    }
                    MouseButton::Button3 => {
                        window_state.middle_button_pressed = true
                    }
                    _ => (),
                }
            }
            WindowEvent::MouseButton(button, Action::Release, _) => {
                match button {
                    MouseButton::Button1 => {
                        window_state.left_button_pressed = false
                    }
                    MouseButton::Button2 => {
                        window_state.right_button_pressed = false
                    }
                    MouseButton::Button3 => {
                        window_state.middle_button_pressed = false
                    }
                    _ => (),
                }
            }
            WindowEvent::Key(key, _, Action::Press, _) => {
                window_state.keyboard_button_pressed = true;
                window_state.pressed_keys.insert(key);
            }
            WindowEvent::Key(key, _, Action::Release, _) => {
                window_state.keyboard_button_pressed = false;
                window_state.pressed_keys.remove(&key);
            }
            WindowEvent::CursorPos(x, y) => {
                window_state.mouse_pos.x = x as f32 - 0.5 * window_state.width;
                window_state.mouse_pos.y = 0.5 * window_state.height - y as f32;
            }
            WindowEvent::Close => {
                window_state.should_close = true;
            }
            WindowEvent::FramebufferSize(width, height) => {
                window_state.width = width as f32;
                window_state.height = height as f32;
            }
            _ => (),
        }
        Ok(())
    }

    /// Toggle application fullscreen.
    ///
    /// If the window is currently windowed then swap to fullscreen using
    /// whatever the primary monitor advertises as the primary video mode.
    ///
    /// If the window is currently fullscreen, then swap to windowed and
    /// restore the window's previous size and location.
    fn toggle_glfw_fullscreen(
        &mut self,
        window_state: &mut WindowState,
    ) -> Result<()> {
        let is_fullscreen = self.is_glfw_window_fullscreen();
        if is_fullscreen {
            // Switch to windowed mode.
            self.set_monitor(
                WindowMode::Windowed,
                window_state.window_x,
                window_state.window_y,
                window_state.windowed_width as u32,
                window_state.windowed_height as u32,
                None,
            );
        } else {
            // Switch to fullscreen mode.
            // Record the size and position of the non-fullscreen window
            // before switching modes.
            (window_state.windowed_width, window_state.windowed_height) =
                self.get_size();
            (window_state.window_x, window_state.window_y) = self.get_pos();

            let mut glfw = self.glfw.clone();
            glfw.with_primary_monitor_mut(|_, monitor_opt| -> Result<()> {
                let monitor = monitor_opt
                    .context("Unable to determine the primary monitor!")?;
                let video_mode = monitor
                    .get_video_mode()
                    .context("Unable to get a primary video mode!")?;
                self.set_monitor(
                    WindowMode::FullScreen(monitor),
                    0,
                    0,
                    video_mode.width,
                    video_mode.height,
                    Some(video_mode.refresh_rate),
                );
                Ok(())
            })?;
        }
        window_state.is_fullscreen = self.is_glfw_window_fullscreen();
        Ok(())
    }

    fn is_glfw_window_fullscreen(&self) -> bool {
        self.with_window_mode(|mode| match mode {
            WindowMode::Windowed => false,
            WindowMode::FullScreen(_) => true,
        })
    }
}
