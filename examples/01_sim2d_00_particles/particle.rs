use sim2d::{
    graphics::renderer::primitive::Vertex,
    math::{vec2, Vec2f},
};

pub struct Particle {
    pub position: Vec2f,
    pub position_previous: Vec2f,
    pub acceleration: Vec2f,
}

impl Particle {
    pub fn new(pos: Vec2f) -> Self {
        Self {
            position: pos,
            position_previous: pos,
            acceleration: Vec2f::zeros(),
        }
    }

    pub fn integrate(&mut self, dt: f32) {
        let velocity = self.velocity();
        self.position_previous = self.position;

        self.position += velocity + self.acceleration * (dt * dt);
        self.acceleration = vec2(0.0, 0.0);
    }

    pub fn accelerate(&mut self, acceleration: Vec2f) {
        self.acceleration += acceleration;
    }

    pub fn velocity(&self) -> Vec2f {
        self.position - self.position_previous
    }

    pub fn vertex(&self) -> Vertex {
        Vertex::new(self.position, self.velocity(), [1.0, 1.0, 1.0, 0.25])
    }
}
