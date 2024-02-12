pub type Vec2<T> = nalgebra::Vector2<T>;
pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2ui = Vec2<u32>;

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}
