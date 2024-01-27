use {
    anyhow::Result,
    sim2d::application::{glfw_application_main, GLFWApplication},
};

struct MyApp {}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title("Example 00");
        Ok(MyApp {})
    }

    fn handle_event(&mut self, event: glfw::WindowEvent) -> Result<()> {
        log::info!("Handled event {:?}", event);
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    glfw_application_main::<MyApp>()
}
