use {
    super::{vec2, Vec2},
    sim2d::graphics::renderer::primitive::Vertex,
};

pub struct Line {
    pub start: Particle,
    pub end: Particle,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            start: Particle::new(start),
            end: Particle::new(end),
        }
    }

    pub fn vertices(&self) -> [Vertex; 2] {
        //let max_mag = 50.0;
        //let mag = (self.start.position - self.end.position).magnitude();
        let speed = self.start.velocity().magnitude();

        //let t = (1.0 - (mag / max_mag)).clamp(0.0, 1.0);

        let v = (speed / 20.0).clamp(0.0, 1.0);
        let r = v;
        let b = 1.0 - v;
        let a = (v * 0.1).max(0.005);
        [
            Vertex::new(
                self.start.position,
                self.start.velocity(),
                [r, r, b, a],
            ),
            Vertex::new(self.end.position, self.end.velocity(), [r, r, b, a]),
        ]
    }
}

pub struct Particle {
    pub position: Vec2,
    pub position_previous: Vec2,
    pub acceleration: Vec2,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            position_previous: pos,
            acceleration: vec2(0.0, 0.0),
        }
    }

    pub fn integrate(&mut self, dt: f32) {
        let velocity = self.velocity();
        self.position_previous = self.position;

        self.position += (0.99 * velocity) + self.acceleration * (dt * dt);
        self.acceleration = vec2(0.0, 0.0);
    }

    pub fn accelerate(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }

    pub fn velocity(&self) -> Vec2 {
        self.position - self.position_previous
    }
}
