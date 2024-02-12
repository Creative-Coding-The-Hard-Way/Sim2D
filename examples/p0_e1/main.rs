mod particle;

use {
    anyhow::Result,
    ash::vk,
    glfw::{Action, MouseButton, WindowEvent},
    particle::Line,
    rand::prelude::*,
    rayon::iter::{
        IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge,
        ParallelExtend, ParallelIterator,
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
    lines: Vec<Line>,
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
                topology: vk::PrimitiveTopology::LINE_LIST,
            },
        )?;

        let screen_size = vec2(1920.0, 1080.0);
        let w = screen_size.x * 0.5;
        let h = screen_size.y * 0.5;
        let lines: Vec<_> = (0..100_000)
            .par_bridge()
            .map_init(rand::thread_rng, |rng, _| {
                Line::new(
                    vec2(rng.gen_range(-w..w), rng.gen_range(-h..h)),
                    vec2(rng.gen_range(-w..w), rng.gen_range(-h..h)),
                )
            })
            .collect();

        Ok(MyApp {
            rc,
            renderer,
            last_update: Instant::now(),
            mouse_pos: vec2(0.0, 0.0),
            screen_size,
            pressed: false,
            vertices: Vec::with_capacity(lines.len() * 2),
            lines,
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
                self.lines.par_iter_mut().for_each_init(
                    rand::thread_rng,
                    |rng, line| {
                        let offset = std::f32::consts::TAU / 360.0;
                        let a = rng.gen_range(0.0..std::f32::consts::TAU);
                        let s = mouse_pos + vec2(r * a.cos(), r * a.sin());
                        let e = mouse_pos
                            + vec2(
                                r * (a + offset).cos(),
                                r * (a + offset).sin(),
                            );
                        line.start.position = s;
                        line.start.position_previous = s;
                        line.end.position = e;
                        line.end.position_previous = e;
                    },
                );
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
        self.lines
            .par_iter_mut()
            .map(|line| {
                let dir = line.start.position - line.end.position;
                let len = dir.magnitude();
                let delta = len - 50.0;
                if delta > 0.0 {
                    let nd = 0.5 * delta * (dir / len);
                    line.start.position -= nd;
                    line.end.position += nd;
                }
                line
            })
            .flat_map(|line| [&mut line.start, &mut line.end])
            .for_each(|particle| {
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
            });

        self.vertices.clear();
        self.vertices.par_extend(
            self.lines.par_iter().map(|line| line.vertices()).flatten(),
        );

        self.renderer.publish_vertices(&self.rc, &self.vertices)
    }

    fn shut_down(&mut self) -> Result<()> {
        self.renderer.shut_down()
    }
}

fn main() {
    glfw_application_main::<MyApp>();
}
