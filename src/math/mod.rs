pub type Vec2<T> = nalgebra::Vector2<T>;
pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2ui = Vec2<u32>;
pub type Mat4<T> = nalgebra::Matrix4<T>;
pub type Mat4f = Mat4<f32>;

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}

/// Compute the orthographic projection matrix which maps a screen's coordinates
/// into a Euclidian plane with the specified dimensions.
#[rustfmt::skip]
pub fn symmetric_ortho<T>(dimensions: Vec2<T>) -> Mat4f
where
    T: nalgebra::Scalar + num_traits::AsPrimitive<f32>,
{
    Mat4f::new(
        2.0 / dimensions.x.as_(), 0.0, 0.0, 0.0, // row 1
        0.0, -2.0 / dimensions.y.as_(), 0.0, 0.0, // row 2
        0.0, 0.0, 1.0, 0.0, // row 3
        0.0, 0.0, 0.0, 1.0, // row 4
    )
}
