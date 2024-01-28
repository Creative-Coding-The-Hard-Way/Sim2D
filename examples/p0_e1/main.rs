use {
    anyhow::Result,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::vulkan::{
            render_context::{Instance, RenderContext, Surface},
            swapchain::Swapchain,
        },
    },
};

struct MyApp {
    rc: RenderContext,
    swapchain: Swapchain,
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

        let (w, h) = window.get_framebuffer_size();
        let swapchain = Swapchain::new(&rc, (w as u32, h as u32))?;

        Ok(MyApp { rc, swapchain })
    }

    fn destroy(&mut self) -> Result<()> {
        unsafe {
            // Wait for all operations to finish.
            self.rc.device.device_wait_idle()?;

            // Destroy everything.
            self.swapchain.destroy();
            self.rc.destroy();
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    glfw_application_main::<MyApp>()
}
