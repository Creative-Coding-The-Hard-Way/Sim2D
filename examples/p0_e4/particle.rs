use {
    super::{vec2, Vec2},
    sim2d::graphics::renderer::primitive::Vertex,
};

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

        self.position += velocity + self.acceleration * (dt * dt);
        self.acceleration = vec2(0.0, 0.0);
    }

    pub fn accelerate(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }

    pub fn velocity(&self) -> Vec2 {
        self.position - self.position_previous
    }

    pub fn vertex(&self) -> Vertex {
        Vertex::new(self.position, self.velocity(), [1.0, 1.0, 1.0, 0.25])
    }
}
