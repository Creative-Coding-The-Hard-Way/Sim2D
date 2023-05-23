use {
    anyhow::Result,
    rand::Rng,
    sim2d::{
        application::{Application, Sketch},
        graphics::vulkan_api::{TextureAtlas, TextureId},
        math::Vec2,
        sim2d::Sim2D,
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
    bunny: TextureId,
    sprites: Vec<Sprite>,
}

impl Sketch for BunnyMark {
    fn new(sim: &mut Sim2D) -> Result<Self> {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];
        Ok(Self::default())
    }

    fn preload(&mut self, texture_atlas: &mut TextureAtlas) -> Result<()> {
        self.bunny = texture_atlas.load_file("examples/e02/bunny.png");
        Ok(())
    }

    fn mouse_released(&mut self, sim: &mut Sim2D) -> Result<()> {
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
        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) -> Result<()> {
        sim.g.texture = self.bunny;
        for sprite in &mut self.sprites {
            sprite.update(sim.dt());
            sprite.constrain(sim);
            sprite.draw(sim);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Application::<BunnyMark>::run()
}
