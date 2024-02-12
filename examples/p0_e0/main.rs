mod particle;

use {
    anyhow::Result,
    ash::vk,
    glfw::{Action, MouseButton, WindowEvent},
    particle::Particle,
    rand::prelude::*,
    rayon::iter::{
        IndexedParallelIterator, IntoParallelRefMutIterator, ParallelBridge,
        ParallelIterator,
    },
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
    },
    std::time::Instant,
};

// Some convenient typedefs
type Vec2 = nalgebra::Vector2<f32>;
fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

struct MyApp {
    // Graphics resources
    rc: RenderContext,
    renderer: AsyncRenderer<InterpolatedPrimitivesRenderer>,

    // Logical Resources
    last_update: Instant,
    particles: Vec<Particle>,
    mouse_pos: Vec2,
    screen_size: Vec2,
    pressed: bool,
    vertices: Vec<Vertex>,
}

impl GLFWApplication for MyApp {
    fn new(window: &mut glfw::Window) -> Result<Self> {
        window.set_title("CPU Particles");
        window.set_size(1920, 1080);

        let rc = RenderContext::frow_glfw_window(window)?;
        let renderer = AsyncRenderer::new(
            &rc,
            Parameters {
                topology: vk::PrimitiveTopology::POINT_LIST,
            },
        )?;

        let screen_size = vec2(1920.0, 1080.0);
        let particles: Vec<_> = (0..100_000)
            .par_bridge()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let w = screen_size.x * 0.5;
                let h = screen_size.y * 0.5;
                let x = rng.gen_range(-w..w);
                let y = rng.gen_range(-h..h);
                Particle::new(vec2(x, y))
            })
            .collect();

        Ok(MyApp {
            rc,
            renderer,
            last_update: Instant::now(),
            mouse_pos: vec2(0.0, 0.0),
            screen_size,
            pressed: false,
            vertices: Vec::with_capacity(particles.len()),
            particles,
        })
    }

    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        match event {
            WindowEvent::FramebufferSize(w, h) => {
                self.renderer.set_projection([
                    [2.0 / *w as f32, 0.0, 0.0, 0.0],
                    [0.0, -2.0 / *h as f32, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])?;
                self.renderer.framebuffer_resized((*w as u32, *h as u32))?;
                self.screen_size = vec2(*w as f32, *h as f32);
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Press,
                _,
            ) => {
                self.pressed = true;
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Release,
                _,
            ) => {
                self.pressed = false;
            }
            WindowEvent::MouseButton(
                MouseButton::Button2,
                Action::Release,
                _,
            ) => {
                let mouse_pos = self.mouse_pos;
                let r = 100.0;
                self.particles.par_iter_mut().for_each(|particle| {
                    let mut rng = rand::thread_rng();
                    let a = rng.gen_range(0.0..std::f32::consts::TAU);
                    let v = mouse_pos + vec2(r * a.cos(), r * a.sin());
                    particle.position = v;
                    particle.position_previous = v;
                });
            }
            WindowEvent::CursorPos(m_x, m_y) => {
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
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();
        self.last_update = now;

        let mouse_pressed = self.pressed;
        let mouse_pos = self.mouse_pos;
        let dim = self.screen_size / 2.0;
        self.particles
            .par_iter_mut()
            .map(|particle| {
                // apply constraints
                particle.position.x = particle.position.x.clamp(-dim.x, dim.x);
                particle.position.y = particle.position.y.clamp(-dim.y, dim.y);

                // update
                if mouse_pressed {
                    let d = mouse_pos - particle.position;
                    let dn = d.normalize();
                    let mag = d.magnitude().powf(1.2).max(10.0);

                    let accel = dn * (500_000.0 / mag);
                    particle.accelerate(accel);
                }
                particle.integrate(dt);

                // compute the vertex
                particle.vertex()
            })
            .collect_into_vec(&mut self.vertices);

        self.renderer.publish_vertices(&self.rc, &self.vertices)
    }

    fn shut_down(&mut self) -> Result<()> {
        self.renderer.shut_down()
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
