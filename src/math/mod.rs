//! Mathematical primitives and operations.

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

pub type Mat4 = Matrix4<f32>;
pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

/// Build an orthographic projection matrix which projects into Vulkan device
/// coordinates (e.g. x in [-1, 1], y in [-1, 1], and z in [0, 1].
/// See 'View Volume' in the glossary: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#glossary
#[rustfmt::skip]
pub fn ortho_projection(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    znear: f32,
    zfar: f32,
) -> Mat4 {
    let w = right - left;
    let h = top - bottom;
    let d = zfar - znear;
    Mat4::new(
        2.0 / w,  0.0    , 0.0    , -1.0 * (right + left)/w,
        0.0    , -2.0 / h, 0.0    , (top + bottom)/h       ,
        0.0    ,  0.0    , 1.0 / d, -1.0 * znear / d       ,
        0.0    ,  0.0    , 0.0    , 1.0                    ,
    )
}

#[cfg(test)]
mod test {
    use {super::*, approx::assert_relative_eq};

    #[test]
    fn test_ortho_projection() {
        let left = -20.0;
        let right = 39.0;
        let top = 100.0;
        let bottom = -234.0;
        let znear = 0.0;
        let zfar = 100.0;
        let proj = ortho_projection(left, right, bottom, top, znear, zfar);

        assert_relative_eq!(-1.0, (proj * Vec4::new(left, 0.0, 0.0, 1.0)).x);
        assert_relative_eq!(1.0, (proj * Vec4::new(right, 0.0, 0.0, 1.0)).x);
        assert_relative_eq!(-1.0, (proj * Vec4::new(0.0, top, 0.0, 1.0)).y);
        assert_relative_eq!(1.0, (proj * Vec4::new(0.0, bottom, 0.0, 1.0)).y);
        assert_relative_eq!(0.0, (proj * Vec4::new(0.0, 0.0, znear, 1.0)).z);
        assert_relative_eq!(1.0, (proj * Vec4::new(0.0, 0.0, zfar, 1.0)).z);
    }
}
