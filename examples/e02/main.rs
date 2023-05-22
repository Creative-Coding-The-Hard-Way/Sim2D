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
        sim.g.rect_centered(self.pos.x, self.pos.y, 25.0, 40.0);
    }
}

#[derive(Default)]
struct HelloG2D {
    gasp: TextureId,
    sprites: Vec<Sprite>,
}

impl Sketch for HelloG2D {
    fn new(sim: &mut Sim2D) -> Result<Self> {
        sim.g.clear_color = [0.0, 0.0, 0.0, 1.0];

        let mut rng = rand::thread_rng();
        let half_w = sim.w.width() / 2.0;
        let half_h = sim.w.height() / 2.0;

        let sprites = (0..10_000)
            .map(|_| Sprite {
                pos: Vec2::new(
                    rng.gen_range(-half_w..half_w),
                    rng.gen_range(-half_h..half_h),
                ),
                vel: Vec2::new(
                    rng.gen_range(-200.0..200.0),
                    rng.gen_range(-200.0..200.0),
                ),
            })
            .collect();

        Ok(Self {
            sprites,
            ..Default::default()
        })
    }

    fn preload(&mut self, texture_atlas: &mut TextureAtlas) -> Result<()> {
        self.gasp = texture_atlas.load_file("examples/e02/Gasp.png");
        Ok(())
    }

    fn update(&mut self, sim: &mut Sim2D) -> Result<()> {
        sim.g.texture = self.gasp;

        for sprite in &mut self.sprites {
            sprite.update(sim.dt());
            sprite.update(sim.dt());
            sprite.update(sim.dt());
            sprite.constrain(sim);
            sprite.draw(sim);
        }

        sim.g.rect_centered(
            sim.w.mouse_pos().x,
            sim.w.mouse_pos().y,
            200.0,
            200.0,
        );
        Ok(())
    }
}

fn main() -> Result<()> {
    Application::<HelloG2D>::run()
}
