mod particle;

use {
    anyhow::Result,
    ash::vk,
    particle::Particle,
    rand::prelude::*,
    rayon::iter::{
        IndexedParallelIterator, IntoParallelRefMutIterator, ParallelBridge,
        ParallelIterator,
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
    vertices: Vec<Vertex>,
}

impl Sim2D for MyApp {
    fn new(rc: RenderContext, state: &WindowState) -> Result<Self> {
        let renderer = AsyncRenderer::<InterpolatedPrimitivesRenderer>::new(
            &rc,
            Parameters {
                topology: vk::PrimitiveTopology::POINT_LIST,
            },
        )?;

        let particles: Vec<_> = (0..100_000)
            .par_bridge()
            .map_init(rand::thread_rng, |rng, _| {
                let limits = state.size() * 0.5;
                let x = rng.gen_range(-limits.x..limits.x);
                let y = rng.gen_range(-limits.y..limits.y);
                Particle::new(vec2(x, y))
            })
            .collect();

        Ok(MyApp {
            rc,
            renderer,
            last_update: Instant::now(),
            vertices: Vec::with_capacity(particles.len()),
            particles,
        })
    }

    fn resized(&mut self, state: &WindowState) -> Result<()> {
        let size = state.size();
        self.renderer.set_projection([
            [2.0 / size.x, 0.0, 0.0, 0.0],
            [0.0, -2.0 / size.y, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])?;
        let fb_size = state.framebuffer_size();
        self.renderer.framebuffer_resized((fb_size.x, fb_size.y))?;
        Ok(())
    }

    fn mouse_released(
        &mut self,
        state: &WindowState,
        button: MouseButton,
    ) -> Result<()> {
        if button == MouseButton::Right {
            let mouse = state.mouse().component_mul(state.size());
            let r = 100.0;
            self.particles.par_iter_mut().for_each(|particle| {
                let mut rng = rand::thread_rng();
                let a = rng.gen_range(0.0..std::f32::consts::TAU);
                let v = mouse + vec2(r * a.cos(), r * a.sin());
                particle.position = v;
                particle.position_previous = v;
            });
        }
        Ok(())
    }

    fn update(&mut self, state: &WindowState) -> Result<()> {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();
        self.last_update = now;

        let mouse_pressed = state.is_button_pressed(MouseButton::Left);
        let mouse = state.mouse().component_mul(state.size());
        let dim = state.size() / 2.0;
        self.particles
            .par_iter_mut()
            .map(|particle| {
                // apply constraints
                particle.position.x = particle.position.x.clamp(-dim.x, dim.x);
                particle.position.y = particle.position.y.clamp(-dim.y, dim.y);

                // update
                if mouse_pressed {
                    let d = mouse - particle.position;
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
    MyApp::main()
}
