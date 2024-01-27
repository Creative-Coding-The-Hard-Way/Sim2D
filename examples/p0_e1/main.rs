use {
    anyhow::Result,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::vulkan::instance::Instance,
    },
};

struct MyApp {
    instance: Instance,
}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title("Example 01");

        let instance = Instance::new("Example 01", &[])?;
        log::info!("Vulkan Instance created! \n{:#?}", instance);

        Ok(MyApp { instance })
    }

    fn destroy(&mut self) -> Result<()> {
        unsafe {
            self.instance.destroy();
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    glfw_application_main::<MyApp>()
}
