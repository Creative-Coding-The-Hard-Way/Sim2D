use {
    anyhow::Result,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::{
            renderer::{
                triangles::{Triangles, Vertex, WritableVertices},
                RenderEvents,
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
    render_thread_handle: Option<JoinHandle<Result<()>>>,
    render_thread_running: Arc<AtomicBool>,
    writable_vertices: WritableVertices,
    render_events_send: Sender<RenderEvents>,
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

        let render_thread_running = Arc::new(AtomicBool::new(true));
        let (render_thread_handle, writable_vertices, render_events_send) = {
            let (render_events_send, render_events_rcv) =
                std::sync::mpsc::channel::<RenderEvents>();

            let running = render_thread_running.clone();
            let rc2 = rc.clone();
            let (w, h) = window.get_framebuffer_size();
            let (mut triangles, writable_vertices) =
                Triangles::new(rc2, (w as u32, h as u32))?;

            (
                std::thread::spawn(move || -> Result<()> {
                    while running.load(Ordering::Relaxed) {
                        if let Ok(RenderEvents::FramebufferResized(w, h)) =
                            render_events_rcv.try_recv()
                        {
                            triangles.rebuild_swapchain((w, h))?
                        }

                        triangles.draw()?;
                    }

                    triangles.shut_down()?;
                    Ok(())
                }),
                writable_vertices,
                render_events_send,
            )
        };

        Ok(MyApp {
            rc,
            render_thread_handle: Some(render_thread_handle),
            render_thread_running,
            writable_vertices,
            render_events_send,
            start_time: Instant::now(),
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        if let &glfw::WindowEvent::FramebufferSize(width, height) = event {
            self.render_events_send
                .send(RenderEvents::FramebufferResized(
                    width as u32,
                    height as u32,
                ))?;
        }
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
        self.render_thread_running.store(false, Ordering::Relaxed);
        self.render_thread_handle.take().unwrap().join().unwrap()?;
        Ok(())
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
