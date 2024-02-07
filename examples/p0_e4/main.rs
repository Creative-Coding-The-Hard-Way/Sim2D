use {
    anyhow::Result,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::{
            renderer::{
                triangles::{Triangles, Vertex, WritableVertices},
                JoinableRenderer, RenderEvents,
            },
            vulkan::{
                raii,
                render_context::{Instance, RenderContext},
            },
        },
    },
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::Sender,
            Arc,
        },
        thread::JoinHandle,
        time::Instant,
    },
};

struct MyApp {
    rc: RenderContext,
    writable_vertices: WritableVertices,
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

        let (renderer, writable_vertices) =
            JoinableRenderer::new::<Triangles>(&rc)?;

        Ok(MyApp {
            rc,
            writable_vertices,
            renderer,
            start_time: Instant::now(),
        })
    }

    fn handle_event(&mut self, _event: &glfw::WindowEvent) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let mut writable = self.writable_vertices.wait_for_vertex_buffer()?;

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

        self.writable_vertices.publish_update(writable)?;

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
