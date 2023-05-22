use {
    super::WindowState,
    crate::math::Vec2,
    anyhow::{Context, Result},
    glfw::{MouseButton, WindowEvent, WindowMode},
};

impl WindowState {
    /// Create a new WindowState based on a GLFW window.
    pub(crate) fn from_glfw_window(window: &glfw::Window) -> Self {
        let (window_x, window_y) = window.get_pos();
        let (w, h) = window.get_size();
        let (mouse_x, mouse_y) = window.get_cursor_pos();
        Self {
            toggle_fullscreen: false,
            is_fullscreen: Self::is_glfw_window_fullscreen(window),
            needs_resized: false,
            should_close: window.should_close(),

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
        }
    }

    /// Update a GLFW window to reflect the current state.
    pub(crate) fn update_window_to_match(
        &mut self,
        window: &mut glfw::Window,
    ) -> Result<()> {
        if self.toggle_fullscreen {
            self.toggle_fullscreen = false;
            self.toggle_glfw_fullscreen(window)?;
        }

        if self.needs_resized {
            self.needs_resized = false;
            window.set_size(self.width as i32, self.height as i32);
        }

        window.set_should_close(self.should_close);

        Ok(())
    }

    pub(crate) fn handle_event(
        &mut self,
        window_event: &WindowEvent,
    ) -> Result<()> {
        match *window_event {
            WindowEvent::MouseButton(button, glfw::Action::Press, _) => {
                match button {
                    MouseButton::Button1 => self.left_button_pressed = true,
                    MouseButton::Button2 => self.right_button_pressed = true,
                    MouseButton::Button3 => self.middle_button_pressed = true,
                    _ => (),
                }
            }
            WindowEvent::MouseButton(button, glfw::Action::Release, _) => {
                match button {
                    MouseButton::Button1 => self.left_button_pressed = false,
                    MouseButton::Button2 => self.right_button_pressed = false,
                    MouseButton::Button3 => self.middle_button_pressed = false,
                    _ => (),
                }
            }
            WindowEvent::CursorPos(x, y) => {
                self.mouse_pos.x = x as f32 - 0.5 * self.width;
                self.mouse_pos.y = 0.5 * self.height - y as f32;
            }
            WindowEvent::Close => {
                self.should_close = true;
            }
            WindowEvent::FramebufferSize(width, height) => {
                self.width = width as f32;
                self.height = height as f32;
            }
            _ => (),
        }
        Ok(())
    }

    fn is_glfw_window_fullscreen(window: &glfw::Window) -> bool {
        window.with_window_mode(|mode| match mode {
            WindowMode::Windowed => false,
            WindowMode::FullScreen(_) => true,
        })
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
        window: &mut glfw::Window,
    ) -> Result<()> {
        let is_fullscreen = Self::is_glfw_window_fullscreen(window);
        if is_fullscreen {
            // Switch to windowed mode.
            window.set_monitor(
                WindowMode::Windowed,
                self.window_x,
                self.window_y,
                self.windowed_width as u32,
                self.windowed_height as u32,
                None,
            );
        } else {
            // Switch to fullscreen mode.
            // Record the size and position of the non-fullscreen window
            // before switching modes.
            (self.windowed_width, self.windowed_height) = window.get_size();
            (self.window_x, self.window_y) = window.get_pos();

            let mut glfw = window.glfw.clone();
            glfw.with_primary_monitor_mut(|_, monitor_opt| -> Result<()> {
                let monitor = monitor_opt
                    .context("Unable to determine the primary monitor!")?;
                let video_mode = monitor
                    .get_video_mode()
                    .context("Unable to get a primary video mode!")?;
                window.set_monitor(
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
        self.is_fullscreen = Self::is_glfw_window_fullscreen(window);
        Ok(())
    }
}
