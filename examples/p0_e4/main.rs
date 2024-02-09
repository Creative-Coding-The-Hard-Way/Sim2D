use {
    anyhow::Result,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::{
            renderer::{
                triangles::{Triangles, TrianglesApi, Vertex},
                JoinableRenderer,
            },
            vulkan::{
                raii,
                render_context::{Instance, RenderContext},
            },
        },
    },
    std::time::Instant,
};

struct MyApp {
    rc: RenderContext,
    triangles: TrianglesApi,
    renderer: JoinableRenderer,
    start_time: Instant,
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

        let surface =
            raii::Surface::from_glfw_window(instance.ash.clone(), window)?;
        let rc = RenderContext::new(instance, surface)?;

        let (renderer, triangles) = JoinableRenderer::new::<Triangles>(&rc)?;

        Ok(MyApp {
            rc,
            triangles,
            renderer,
            start_time: Instant::now(),
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        if let glfw::WindowEvent::FramebufferSize(w, h) = event {
            self.triangles.framebuffer_resized((*w as u32, *h as u32))?;
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let mut writable = self.triangles.wait_for_writable_vertices()?;

        let t = (Instant::now() - self.start_time).as_secs_f32();
        let a0 = t * std::f32::consts::TAU / 10.0;
        let a1 = a0 + std::f32::consts::TAU / 3.0;
        let a2 = a1 + std::f32::consts::TAU / 3.0;
        let r = 1.0;
        writable.write_vertex_data(
            &self.rc,
            &[
                // 1
                Vertex::new(r * a0.cos(), r * a0.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a1.cos(), r * a1.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a2.cos(), r * a2.sin(), 1.0, 0.0, 0.0, 1.0),
                // 2
                Vertex::new(r * a0.cos(), r * a0.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a1.cos(), r * a1.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a2.cos(), r * a2.sin(), 1.0, 0.0, 0.0, 1.0),
                // 3
                Vertex::new(r * a0.cos(), r * a0.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a1.cos(), r * a1.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a2.cos(), r * a2.sin(), 1.0, 0.0, 0.0, 1.0),
            ],
        )?;

        self.triangles.publish_vertices(writable)?;

        Ok(())
    }

    fn shut_down(&mut self) -> Result<()> {
        self.renderer.shut_down()?;
        Ok(())
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
