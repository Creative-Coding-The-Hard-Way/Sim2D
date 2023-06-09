use {
    anyhow::Result,
    rand::Rng,
    sim2d::{
        application::Application,
        graphics::{AssetLoader, Image},
        math::Vec2,
        Sim2D, Sketch,
    },
    std::time::Duration,
};

struct Sprite {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
}

impl Sprite {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    pub fn constrain(&mut self, sim: &Sim2D) {
        let half_w = sim.w.width() / 2.0;
        let half_h = sim.w.height() / 2.0;

        if self.pos.x > half_w || self.pos.x < -half_w {
            self.vel.x *= -1.0;
        }

        if self.pos.y > half_h || self.pos.y < -half_h {
            self.vel.y *= -1.0;
        }

        self.pos.x = self.pos.x.clamp(-half_w, half_w);
        self.pos.y = self.pos.y.clamp(-half_h, half_h);
    }

    pub fn draw(&self, sim: &mut Sim2D) {
        sim.g
            .rect_centered(self.pos, Vec2::new(35.0, 35.0), self.angle);
    }
}

#[derive(Default)]
struct BunnyMark {
    bunny: Image,
    sprites: Vec<Sprite>,
}

impl Sketch for BunnyMark {
    fn preload(&mut self, asset_loader: &mut AssetLoader) -> Result<()> {
        self.bunny =
            asset_loader.load_image_file("examples/e02/bunny.png", true)?;
        Ok(())
    }

    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.5, 0.5, 0.5, 1.0];

        let mut rng = rand::thread_rng();
        self.sprites.extend((0..1_000_000).map(|_| Sprite {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
            ),
            angle: rng.gen_range(0.0..std::f32::consts::TAU),
        }));
        log::info!("Total Sprites: {}", self.sprites.len());
    }

    fn key_pressed(&mut self, _: &mut Sim2D, key: glfw::Key) {
        if key == glfw::Key::Space {
            self.sprites.clear();
        }
    }

    fn update(&mut self, sim: &mut Sim2D) {
        sim.g.fill_color = [1.0, 1.0, 1.0, 1.0];
        sim.g.image = self.bunny;
        for sprite in &mut self.sprites {
            sprite.update(sim.dt());
            sprite.constrain(sim);
            sprite.draw(sim);
        }

        sim.g.fill_color = [0.0, 0.0, 0.0, 1.0];
        let round_ms =
            |d: &Duration| (d.as_secs_f32() * 1000.0 * 100.0).ceil() / 100.0;

        sim.g.text(
            Vec2::new(sim.w.width() * -0.5, sim.w.height() * 0.5),
            format!(
                indoc::indoc!(
                    "
                    |  Frame Time: {}ms
                    |    Sim Time: {}ms
                    | Render Time: {}ms
                    "
                ),
                round_ms(sim.avg_frame_time()),
                round_ms(sim.avg_sim_time()),
                round_ms(sim.avg_render_time()),
            ),
        );
    }
}

fn main() -> Result<()> {
    Application::run(BunnyMark::default())
}
