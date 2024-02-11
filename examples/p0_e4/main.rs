use {
    anyhow::Result,
    rand::prelude::*,
    rayon::prelude::*,
    sim2d::{
        application::{glfw_application_main, GLFWApplication},
        graphics::{
            renderer::{
                triangles::{Triangles, TrianglesApi, Vertex},
                JoinableRenderer,
            },
            vulkan::render_context::RenderContext,
        },
    },
    std::time::Instant,
};

type Vec2 = nalgebra::Vector2<f32>;

fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

struct Particle {
    pub pos: Vec2,
    vel: Vec2,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(0.0, 0.0),
        }
    }

    pub fn integrate(&mut self, acceleration: Vec2, dt: f32) {
        self.vel += acceleration * dt;
        self.vel *= 0.99999;
        self.pos += self.vel * dt;
    }

    pub fn vertex(&self) -> Vertex {
        Vertex::new(self.pos.into(), self.vel.into(), [1.0, 1.0, 1.0, 0.1])
    }
}

struct MyApp {
    rc: RenderContext,
    triangles: TrianglesApi,
    renderer: JoinableRenderer,
    last_update: Instant,
    particles: Vec<Particle>,
    mouse_pos: Vec2,
    screen_size: Vec2,
    done: bool,
}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title("Example 01");
        window.set_size(1920, 1080);

        let rc = RenderContext::frow_glfw_window(window)?;
        let (renderer, triangles) = JoinableRenderer::new::<Triangles>(&rc)?;

        let w = 1920.0 / 2.0;
        let h = 1080.0 / 2.0;
        let mut rng = rand::thread_rng();
        let mut particles = vec![];
        for _ in 0..20_000_000 {
            let x = rng.gen_range(-w..w);
            let y = rng.gen_range(-h..h);
            particles.push(Particle::new(vec2(x, y)));
        }

        Ok(MyApp {
            rc,
            triangles,
            renderer,
            last_update: Instant::now(),
            mouse_pos: vec2(0.0, 0.0),
            screen_size: vec2(1920.0, 1080.0),
            particles,
            done: false,
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        match event {
            glfw::WindowEvent::FramebufferSize(w, h) => {
                self.triangles.set_projection([
                    [2.0 / *w as f32, 0.0, 0.0, 0.0],
                    [0.0, -2.0 / *h as f32, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])?;
                self.triangles.framebuffer_resized((*w as u32, *h as u32))?;
                self.screen_size = vec2(*w as f32, *h as f32);
            }
            glfw::WindowEvent::CursorPos(m_x, m_y) => {
                let x = *m_x as f32;
                let y = *m_y as f32;
                self.mouse_pos.x = x - (self.screen_size.x / 2.0);
                self.mouse_pos.y = (self.screen_size.y / 2.0) - y;
            }
            _ => (),
        }

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        if self.done {
            return Ok(());
        }
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();

        let compute_start = Instant::now();
        {
            let mouse_pos = self.mouse_pos;
            let substeps = 1;
            for _ in 0..substeps {
                self.particles.par_iter_mut().for_each(|particle| {
                    let d = mouse_pos - particle.pos;
                    let dn = d.normalize();
                    let mag = d.magnitude();

                    let accel = dn * (5000.0 / mag);
                    particle.integrate(accel, dt / substeps as f32);
                });
            }
            self.particles
                .par_iter()
                .map(|particle| particle.vertex())
                .collect_into_vec(&mut self.triangles.vertices);
        }
        let compute_dt = Instant::now() - compute_start;

        let publish_start = Instant::now();
        self.triangles.publish_vertices(&self.rc)?;
        let publish_dt = Instant::now() - publish_start;
        self.last_update = now;

        log::info!(
            indoc::indoc! {"
                dt: {}ms
                compute: {}ms
                publish: {}ms
            "},
            (dt * 100_000.0).round() / 100.0,
            (compute_dt.as_secs_f32() * 100_000.0).round() / 100.0,
            (publish_dt.as_secs_f32() * 100_000.0).round() / 100.0,
        );

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
