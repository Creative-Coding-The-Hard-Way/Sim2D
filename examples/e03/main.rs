use {
    anyhow::Result,
    sim2d::{application::Application, math::Vec2, Sim2D, Sketch},
};

#[derive(Default)]
struct LissajousDiagram {
    t: f32,
    next_t: f32,
    radius: f32,
    points: Vec<Vec2>,
}

impl Sketch for LissajousDiagram {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.1, 0.1, 0.2, 1.0];
        self.next_t = 0.2;
    }

    fn key_pressed(&mut self, sim: &mut Sim2D, key: glfw::Key) {
        if key == glfw::Key::Space {
            sim.w.toggle_fullscreen();
        }
    }

    fn update(&mut self, sim: &mut Sim2D) {
        self.t += sim.dt();
        let angle = self.t * std::f32::consts::PI;

        self.radius = (sim.w.height() * 0.5 * 0.75)
            * (angle * 0.2).cos().abs().clamp(0.001, 1.0);

        let current =
            Vec2::new((angle * 3.5).cos(), (angle * 2.1).sin()) * self.radius;

        if self.t > self.next_t {
            self.points.push(current);
            self.next_t = self.t + 0.01;
        }

        if self.points.len() > 100 {
            self.points.remove(0);
        }

        for i in 0..self.points.len() {
            for j in 0..self.points.len() {
                if i == j {
                    continue;
                }
                self.colored_line(sim, self.points[i], self.points[j]);
            }
            self.colored_line(sim, self.points[i], current);
        }

        sim.g.fill_color = [0.0, 0.0, 0.0, 1.0];
        sim2d::ext::draw_fps_panel(sim);
    }
}

impl LissajousDiagram {
    fn colored_line(&self, sim: &mut Sim2D, start: Vec2, end: Vec2) {
        let d2 = (end - start).magnitude_squared();
        let c = 1.0 - (d2 / (self.radius * self.radius));
        sim.g.fill_color = [c, c, c, c];
        sim.g.line(start, end);
    }
}

fn main() -> Result<()> {
    Application::run(LissajousDiagram::default())
}
