use {
    anyhow::Result,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::vulkan::{
            instance::Instance,
            render_context::{RenderContext, Surface},
        },
    },
};

struct MyApp {
    rc: RenderContext,
}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title("Example 01");

        let instance = Instance::new(
            "Example 01",
            &window
                .glfw
                .get_required_instance_extensions()
                .unwrap_or_default(),
        )?;
        log::info!("Vulkan Instance created! \n{:#?}", instance);

        let rc = RenderContext::new(
            instance.clone(),
            Surface::from_glfw_window(window, &instance)?,
        )?;

        Ok(MyApp { rc })
    }

    fn destroy(&mut self) -> Result<()> {
        unsafe {
            self.rc.destroy();
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    glfw_application_main::<MyApp>()
}
