mod particle;

use {
    anyhow::Result,
    ash::vk,
    particle::Line,
    rand::prelude::*,
    rayon::iter::{
        IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge,
        ParallelExtend, ParallelIterator,
    },
    sim2d::{
        application::{MouseButton, Sim2D, WindowState},
        graphics::{
            renderer::{
                primitive::{
                    InterpolatedPrimitivesRenderer, Parameters, Vertex,
                },
                AsyncRenderer,
            },
            vulkan::render_context::RenderContext,
        },
        math::{symmetric_ortho, vec2},
    },
    std::time::Instant,
};

struct MyApp {
    // Graphics resources
    rc: RenderContext,
    renderer: AsyncRenderer<InterpolatedPrimitivesRenderer>,

    // Logical Resources
    last_update: Instant,
    lines: Vec<Line>,
    vertices: Vec<Vertex>,
}

impl Sim2D for MyApp {
    fn new(rc: RenderContext, state: &WindowState) -> Result<Self> {
        state.set_title(module_path!())?;
        let size = state.set_size(vec2(800.0, 600.0))?;

        let renderer = AsyncRenderer::new(
            &rc,
            Parameters {
                topology: vk::PrimitiveTopology::LINE_LIST,
                framebuffer_size: *state.framebuffer_size(),
                projection: symmetric_ortho(size),
            },
        )?;

        let w = size.x * 0.5;
        let h = size.y * 0.5;
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
            vertices: Vec::with_capacity(lines.len() * 2),
            lines,
        })
    }

    fn resized(&mut self, window: &WindowState) -> Result<()> {
        self.renderer
            .set_projection(&symmetric_ortho(*window.size()))?;
        self.renderer
            .framebuffer_resized(*window.framebuffer_size())
    }

    fn mouse_released(
        &mut self,
        window: &WindowState,
        button: sim2d::application::MouseButton,
    ) -> Result<()> {
        if button != MouseButton::Right {
            return Ok(());
        }

        let mouse_pos = window.mouse().component_mul(window.size());
        let r = 100.0;
        self.lines.par_iter_mut().for_each_init(
            rand::thread_rng,
            |rng, line| {
                let offset = std::f32::consts::TAU / 360.0;
                let a = rng.gen_range(0.0..std::f32::consts::TAU);
                let s = mouse_pos + vec2(r * a.cos(), r * a.sin());
                let e = mouse_pos
                    + vec2(r * (a + offset).cos(), r * (a + offset).sin());
                line.start.position = s;
                line.start.position_previous = s;
                line.end.position = e;
                line.end.position_previous = e;
            },
        );
        Ok(())
    }
    fn update(&mut self, window: &WindowState) -> Result<()> {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();
        self.last_update = now;

        let mouse_pressed = window.any_button_pressed();
        let mouse_pos = window.mouse().component_mul(window.size());
        let dim = 0.5 * window.size();
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
    MyApp::main();
}
