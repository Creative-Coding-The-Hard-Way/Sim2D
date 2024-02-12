use {
    anyhow::Result,
    ash::vk,
    glfw::WindowEvent,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::{
            renderer::{
                primitive::{
                    InterpolatedPrimitivesRenderer, Parameters, Vertex,
                },
                AsyncRenderer,
            },
            vulkan::render_context::RenderContext,
        },
        math::{vec2, Vec2f},
    },
    std::time::{Duration, Instant},
};

struct MyApp {
    // Graphics resources
    rc: RenderContext,
    renderer: AsyncRenderer<InterpolatedPrimitivesRenderer>,

    // Logical Resources
    start_time: Instant,
}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title(module_path!());
        window.set_size(1920, 1080);

        let rc = RenderContext::frow_glfw_window(window)?;
        let renderer = AsyncRenderer::new(
            &rc,
            Parameters {
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            },
        )?;

        Ok(MyApp {
            rc,
            start_time: Instant::now(),
            renderer,
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        if let &WindowEvent::FramebufferSize(w, h) = event {
            self.renderer.set_projection([
                [2.0 / w as f32, 0.0, 0.0, 0.0],
                [0.0, -2.0 / h as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])?;
            self.renderer.framebuffer_resized((w as u32, h as u32))?;
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let time = (Instant::now() - self.start_time).as_secs_f32();
        let r = 300.0;
        let a1 = time * std::f32::consts::TAU / 20.0;
        let a2 = a1 + std::f32::consts::TAU / 3.0;
        let a3 = a2 + std::f32::consts::TAU / 3.0;
        self.renderer.publish_vertices(
            &self.rc,
            &[
                Vertex::new(
                    r * vec2(a1.cos(), a1.sin()),
                    Vec2f::zeros(),
                    [1.0, 0.0, 0.0, 1.0],
                ),
                Vertex::new(
                    r * vec2(a2.cos(), a2.sin()),
                    Vec2f::zeros(),
                    [0.0, 1.0, 0.0, 1.0],
                ),
                Vertex::new(
                    r * vec2(a3.cos(), a3.sin()),
                    Vec2f::zeros(),
                    [0.0, 0.0, 1.0, 1.0],
                ),
            ],
        )?;
        std::thread::sleep(Duration::from_millis(8));
        Ok(())
    }

    fn shut_down(&mut self) -> Result<()> {
        self.renderer.shut_down()
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
