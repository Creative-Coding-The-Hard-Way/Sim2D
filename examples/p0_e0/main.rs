use {
    anyhow::Result,
    sim2d::application::{glfw_application_main, GLFWApplication},
};

struct MyApp {}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Self {
        window.set_title("Example 01");
        MyApp {}
    }

    fn handle_event(&mut self, event: glfw::WindowEvent) {
        log::info!("Handled event {:?}", event);
    }

    fn update(&mut self) {}
}

fn main() -> Result<()> {
    glfw_application_main::<MyApp>()
}
