use {
    anyhow::Result,
    rand::Rng,
    sim2d::{
        application::{Application, Sketch},
        graphics::vulkan_api::{TextureAtlas, TextureId},
        math::Vec2,
        sim2d::Sim2D,
    },
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
    bunny: TextureId,
    sprites: Vec<Sprite>,
}

impl Sketch for BunnyMark {
    fn preload(&mut self, texture_atlas: &mut TextureAtlas) {
        self.bunny = texture_atlas.load_file("examples/e02/bunny.png");
    }

    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.5, 0.5, 0.5, 1.0];
    }

    fn mouse_released(&mut self, sim: &mut Sim2D) {
        let mut rng = rand::thread_rng();

        self.sprites.extend((0..20_000).map(|_| Sprite {
            pos: sim.w.mouse_pos(),
            vel: Vec2::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-400.0..50.0),
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
        sim.g.texture = self.bunny;
        for sprite in &mut self.sprites {
            sprite.update(sim.dt());
            sprite.constrain(sim);
            sprite.draw(sim);
        }
    }
}

fn main() -> Result<()> {
    Application::run(BunnyMark::default())
}
