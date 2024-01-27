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

/// Create a Vulkan Window surface from a GLFW Window and Ash library instance.
pub fn create_window_surface(
    window: &glfw::Window,
    ash: &ash::Instance,
) -> Result<ash::vk::SurfaceKHR> {
    let mut surface = ash::vk::SurfaceKHR::null();
    let result = window.create_window_surface(
        ash.handle(),
        std::ptr::null(),
        &mut surface,
    );
    if result != ash::vk::Result::SUCCESS {
        anyhow::bail!(
            "Unable to create the Vulkan SurfaceKHR with GLFW! {:?}",
            result
        );
    }
    Ok(surface)
}

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
                .unwrap_or(vec![]),
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
