mod glfw_application;
mod window_state;

use {crate::graphics::vulkan::render_context::RenderContext, anyhow::Result};
pub use {
    glfw_application::{glfw_application_main, GLFWApplication},
    window_state::{MouseButton, WindowEvent, WindowState},
};

/// The standard application entrypoint.
pub trait Sim2D {
    /// Create a new Sim2D application.
    fn new(rc: RenderContext, window_state: &WindowState) -> Result<Self>
    where
        Self: Sized;

    /// Handle a window event.
    #[allow(unused_variables)]
    fn handle_event(
        &mut self,
        window_state: &WindowState,
        event: &WindowEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// Update the application.
    #[allow(unused_variables)]
    fn update(&mut self, window_state: &WindowState) -> Result<()> {
        Ok(())
    }

    fn shut_down(&mut self) -> Result<()> {
        Ok(())
    }

    /// The application entrypoint for this Sim2D app.
    fn main()
    where
        Self: Sized + Send + 'static,
    {
        glfw_application_main::<GlfwSim2DApp<Self>>()
    }
}

struct GlfwSim2DApp<S: Sim2D> {
    sim: S,
    window_state: WindowState,
}

impl<S: Sim2D> GLFWApplication for GlfwSim2DApp<S> {
    fn new(window: &mut glfw::Window) -> Result<Self>
    where
        Self: Sized,
    {
        let window_state = WindowState::new(window);
        let rc = RenderContext::frow_glfw_window(window)?;
        Ok(Self {
            sim: S::new(rc, &window_state)?,
            window_state,
        })
    }

    fn handle_event(&mut self, glfw_event: &glfw::WindowEvent) -> Result<()> {
        self.window_state
            .handle_event(glfw_event)
            .map(|window_event| {
                self.sim.handle_event(&self.window_state, &window_event)
            })
            .unwrap_or(Ok(()))
    }

    fn update(&mut self) -> Result<()> {
        self.sim.update(&self.window_state)
    }

    fn shut_down(&mut self) -> Result<()> {
        self.sim.shut_down()
    }
}
