use {
    anyhow::Result,
    sim2d::{
        application::{Application, Sketch},
        math::Vec2,
        Sim2D,
    },
};

#[derive(Default)]
struct HelloG2D {
    t: f32,
}

impl Sketch for HelloG2D {
    fn setup(&mut self, sim: &mut Sim2D) {
        sim.g.clear_color = [0.5, 0.5, 0.5, 1.0];
    }

    fn update(&mut self, sim: &mut Sim2D) {
        self.t += sim.dt();

        let angle = self.t * std::f32::consts::TAU / 10.0;

        sim.g.fill_color = [0.0, 0.0, 0.0, 1.0];
        sim.g.rect_centered(
            Vec2::new(200.0, 200.0),
            Vec2::new(50.0, 100.0),
            angle,
        );
        sim.g.rect_centered(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, sim.w.height()),
            0.0,
        );
        sim.g.rect_centered(
            Vec2::new(0.0, 0.0),
            Vec2::new(sim.w.width(), 1.0),
            0.0,
        );

        sim.g.line(Vec2::new(0.0, 0.0), sim.w.mouse_pos());

        sim.g.fill_color = [1.0, 1.0, 1.0, 1.0];
        sim.g
            .rect(Vec2::new(0.0, 0.0), Vec2::new(200.0, 200.0), angle);
    }
}

fn main() -> Result<()> {
    Application::run(HelloG2D::default())
}
