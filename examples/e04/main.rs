use {
    anyhow::Result,
    sim2d::{
        application::Application,
        graphics::{AssetLoader, Image},
        math::Vec2,
        Sim2D, Sketch,
    },
};

#[derive(Clone, Debug, Default)]
struct Planet {
    pos: Vec2,
    prev_pos: Vec2,
    tail: Vec<Vec2>,
    net_force: Vec2,
    mass: f32,
    radius: f32,
    collision_count: f32,
    circle_texture: Image,
}

impl Planet {
    fn apply_force(&mut self, force: Vec2) {
        self.net_force += force;
    }

    fn constrain(&mut self, sim: &Sim2D) {
        let half_w = (sim.w.width() - self.radius) / 2.0;
        let half_h = (sim.w.height() - self.radius) / 2.0;

        self.pos.x = self.pos.x.clamp(-half_w, half_w);
        self.pos.y = self.pos.y.clamp(-half_h, half_h);
    }

    fn integrate(&mut self, dt: f32) {
        let acceleration = self.net_force / self.mass;
        let next_pos =
            (2.0 * self.pos - self.prev_pos) + (acceleration * dt * dt);

        self.net_force = Vec2::new(0.0, 0.0);
        self.tail.push(self.prev_pos);
        if self.tail.len() > 100 {
            self.tail.remove(0);
        }

        self.prev_pos = self.pos;
        self.pos = next_pos;
    }

    fn draw(&self, sim: &mut Sim2D) {
        sim.g.image = self.circle_texture;
        let c = (self.collision_count / 10.0).clamp(0.0, 1.0);
        sim.g.fill_color = [c, 0.0, 1.0 - c, 1.0];
        sim.g.rect_centered(
            self.pos,
            Vec2::new(self.radius * 2.0, self.radius * 2.0),
            0.0,
        );
    }

    fn draw_tail(&self, sim: &mut Sim2D) {
        if self.tail.is_empty() {
            return;
        }
        sim.g.image = Image::none();
        let c = (self.collision_count / 10.0).clamp(0.0, 1.0);
        sim.g.fill_color = [c, 0.0, 1.0 - c, 0.2];
        sim.g.line_width = 1.0;

        for i in 0..(self.tail.len() - 1) {
            sim.g.line(self.tail[i], self.tail[i + 1]);
        }
    }
}

#[derive(Default)]
struct NBodySystem {
    circle: Image,
    planets: Vec<Planet>,
    t: f32,
    next_step: f32,
    step: f32,
}

impl Sketch for NBodySystem {
    fn preload(&mut self, asset_loader: &mut AssetLoader) -> Result<()> {
        self.circle =
            asset_loader.load_image_file("examples/e04/Circle.png", true)?;
        Ok(())
    }

    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.0, 0.0, 0.01, 1.0];
        self.step = 0.01;

        self.add_planet(Planet {
            pos: Vec2::new(-200.0, 0.0),
            prev_pos: Vec2::new(-200.0, 120.0 * self.step),
            radius: 30.0,
            mass: 10_000_000.0,
            net_force: Vec2::new(0.0, 0.0),
            circle_texture: self.circle,
            ..Default::default()
        });

        self.add_planet(Planet {
            pos: Vec2::new(200.0, 0.0),
            prev_pos: Vec2::new(200.0, -120.0 * self.step),
            radius: 30.0,
            mass: 10_000_000.0,
            net_force: Vec2::new(0.0, 0.0),
            circle_texture: self.circle,
            ..Default::default()
        });
    }

    fn mouse_pressed(&mut self, sim: &mut Sim2D) {
        if sim.w.is_left_mouse_button_pressed() {
            let count = 8;
            let angle_step = std::f32::consts::TAU / count as f32;
            for i in 0..count {
                let angle = i as f32 * angle_step;
                let start = sim.w.mouse_pos()
                    + ((2.0 + i as f32) / count as f32)
                        * 100.0
                        * Vec2::new(angle.cos(), angle.sin());
                self.add_planet(Planet {
                    pos: start,
                    prev_pos: start,
                    radius: 7.0,
                    mass: 50_000.0,
                    net_force: Vec2::new(0.0, 0.0),
                    circle_texture: self.circle,
                    ..Default::default()
                });
            }
        } else {
            self.add_planet(Planet {
                pos: sim.w.mouse_pos(),
                prev_pos: sim.w.mouse_pos(),
                radius: 30.0,
                mass: 10_000_000.0,
                net_force: Vec2::new(0.0, 0.0),
                circle_texture: self.circle,
                ..Default::default()
            });
        }
    }

    fn key_pressed(&mut self, _sim: &mut Sim2D, _key: glfw::Key) {
        self.planets.clear();
    }

    fn update(&mut self, sim: &mut Sim2D) {
        self.t += sim.dt();

        if self.t > self.next_step {
            self.next_step = self.t + self.step;
            self.integrate(sim);
        }

        for planet in &self.planets {
            planet.draw_tail(sim);
        }

        for planet in &self.planets {
            planet.draw(sim);
        }

        sim.g.fill_color = [1.0, 1.0, 1.0, 1.0];
        sim2d::ext::draw_fps_panel(sim);
    }
}

impl NBodySystem {
    fn integrate(&mut self, sim: &Sim2D) {
        let count = self.planets.len();
        for i in 0..count {
            self.planets[i].collision_count *= 0.5;

            for j in (i + 1)..count {
                let a = self.planets[i].clone();
                let b = self.planets[j].clone();

                let dir = b.pos - a.pos;
                let distance = dir.magnitude();

                let norm_dir = dir / distance;
                let force =
                    norm_dir * (a.mass * b.mass) / (distance * distance);

                self.planets[i].apply_force(force);
                self.planets[j].apply_force(-force);

                let min_distance = a.radius + b.radius;
                if distance >= min_distance {
                    continue;
                }

                let delta = distance - min_distance;
                let i_amount = delta * b.mass / (a.mass + b.mass);
                let j_amount = delta * a.mass / (a.mass + b.mass);

                self.planets[i].pos += norm_dir * i_amount;
                self.planets[j].pos -= norm_dir * j_amount;

                self.planets[i].collision_count += 1.0;
                self.planets[j].collision_count += 1.0;
            }
        }

        for planet in &mut self.planets {
            planet.constrain(sim);
            planet.integrate(self.step);
        }
    }

    fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }
}

fn main() -> Result<()> {
    Application::run(NBodySystem::default())
}
