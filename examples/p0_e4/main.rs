use {
    anyhow::{Context, Result},
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::{
            renderer::triangles::{Triangles, Vertex, VertexBuffer},
            vulkan::{
                raii,
                render_context::{Instance, RenderContext},
            },
        },
    },
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{Receiver, SyncSender},
            Arc,
        },
        thread::JoinHandle,
        time::Instant,
    },
};

struct MyApp {
    render_thread_handle: Option<JoinHandle<Result<()>>>,
    render_thread_running: Arc<AtomicBool>,
    writable_vertex_rcv: Receiver<VertexBuffer>,
    publish_vertices_send: SyncSender<VertexBuffer>,
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
        let (render_thread_handle, writable_vertex_rcv, publish_vertices_send) = {
            let (writable_vertex_send, writable_vertex_rcv) =
                std::sync::mpsc::channel::<VertexBuffer>();
            let (publish_vertices_send, publish_vertices_rcv) =
                std::sync::mpsc::sync_channel::<VertexBuffer>(1);

            let running = render_thread_running.clone();
            let rc2 = rc.clone();
            let (w, h) = window.get_framebuffer_size();

            (
                std::thread::spawn(move || -> Result<()> {
                    let mut triangles =
                        Triangles::new(rc2, (w as u32, h as u32))?;

                    while running.load(Ordering::Relaxed) {
                        if let Some(vertices) =
                            triangles.try_get_writable_buffer()
                        {
                            // a vertex buffer is available for writing
                            writable_vertex_send.send(vertices)?;
                        }

                        if let Ok(vertices) = publish_vertices_rcv.try_recv() {
                            triangles.publish_update(vertices);
                        }

                        triangles.draw()?;
                    }

                    triangles.destroy()?;
                    Ok(())
                }),
                writable_vertex_rcv,
                publish_vertices_send,
            )
        };

        Ok(MyApp {
            render_thread_handle: Some(render_thread_handle),
            render_thread_running,
            writable_vertex_rcv,
            publish_vertices_send,
            start_time: Instant::now(),
        })
    }

    fn handle_event(&mut self, _event: &glfw::WindowEvent) -> Result<()> {
        //if let &glfw::WindowEvent::FramebufferSize(width, height) = event {
        //}
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let mut writable = self
            .writable_vertex_rcv
            .recv()
            .context("Unable to get writable vertex buffer!")?;

        let t = (Instant::now() - self.start_time).as_secs_f32();
        let a0 = t * std::f32::consts::TAU / 10.0;
        let a1 = a0 + std::f32::consts::TAU / 3.0;
        let a2 = a1 + std::f32::consts::TAU / 3.0;
        let r = 1.0;
        unsafe {
            writable.write_vertex_data(&[
                Vertex::new(r * a0.cos(), r * a0.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a1.cos(), r * a1.sin(), 1.0, 0.0, 0.0, 1.0),
                Vertex::new(r * a2.cos(), r * a2.sin(), 1.0, 0.0, 0.0, 1.0),
            ]);
        }

        self.publish_vertices_send
            .send(writable)
            .context("Error while publishing vertex updates!")?;

        Ok(())
    }

    fn destroy(&mut self) -> Result<()> {
        self.render_thread_running.store(false, Ordering::Relaxed);
        self.render_thread_handle.take().unwrap().join().unwrap()?;
        Ok(())
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
