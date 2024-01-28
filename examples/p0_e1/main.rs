mod pipeline;
mod render_pass;

use {
    anyhow::{Context, Result},
    ash::vk,
    pipeline::GraphicsPipeline,
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
    pipeline: pipeline::GraphicsPipeline,
    render_pass: vk::RenderPass,
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

        let render_pass = render_pass::create_render_pass(&rc, &swapchain)?;
        let pipeline = pipeline::GraphicsPipeline::new(&rc, &render_pass)?;

        Ok(MyApp {
            rc,
            swapchain,
            pipeline,
            render_pass,
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        if let &glfw::WindowEvent::FramebufferSize(width, height) = event {
            unsafe {
                self.swapchain
                    .rebuild_swapchain(&self.rc, (width as u32, height as u32))
                    .with_context(|| "Unable to resize the swapchain!")?;
                self.rc.device.destroy_render_pass(self.render_pass, None);
                self.pipeline.destroy(&self.rc);
                self.render_pass =
                    render_pass::create_render_pass(&self.rc, &self.swapchain)
                        .with_context(|| {
                            "Unable to rebuild the render pass!"
                        })?;
                self.pipeline =
                    GraphicsPipeline::new(&self.rc, &self.render_pass)
                        .with_context(|| {
                            "Unable to rebuild the graphics pipeline!"
                        })?;
            }
        }
        Ok(())
    }

    fn destroy(&mut self) -> Result<()> {
        unsafe {
            // Wait for all operations to finish.
            self.rc.device.device_wait_idle()?;

            // Destroy everything.
            self.rc.device.destroy_render_pass(self.render_pass, None);
            self.pipeline.destroy(&self.rc);
            self.swapchain.destroy(&self.rc);
            self.rc.destroy();
        };
        Ok(())
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
