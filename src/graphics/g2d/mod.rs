use crate::{
    graphics::vulkan_api::{BindlessVertex, TextureId},
    math::Vec2,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ShapeVertex {
    pub pos: Vec2,
    pub uv: Vec2,
}

pub struct G2D {
    vertices: Vec<BindlessVertex>,
    indices: Vec<u32>,
    pub clear_color: [f32; 4],
    pub fill_color: [f32; 4],
    pub texture: TextureId,
    pub line_width: f32,
}

impl Default for G2D {
    fn default() -> Self {
        Self {
            vertices: Vec::with_capacity(10_000),
            indices: Vec::with_capacity(10_1000),
            clear_color: [1.0, 1.0, 1.0, 1.0],
            fill_color: [1.0, 1.0, 1.0, 1.0],
            texture: TextureId::no_texture(),
            line_width: 1.0,
        }
    }
}

impl G2D {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let start = Vec2::new(x1, y1);
        let end = Vec2::new(x2, y2);
        let dir = (start - end).normalize();
        let normal = Vec2::new(-dir.y, dir.x);
        let half_n = normal * 0.5 * self.line_width;
        self.quad(start + half_n, end + half_n, start - half_n, end - half_n);
    }

    pub fn rect_centered(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.rect(x - 0.5 * width, y + 0.5 * height, width, height)
    }

    pub fn rect(&mut self, left: f32, top: f32, width: f32, height: f32) {
        let bottom = top - height;
        let right = left + width;
        self.quad(
            Vec2::new(left, top),
            Vec2::new(right, top),
            Vec2::new(left, bottom),
            Vec2::new(right, bottom),
        );
    }

    pub fn quad(
        &mut self,
        top_left: Vec2,
        top_right: Vec2,
        bottom_left: Vec2,
        bottom_right: Vec2,
    ) {
        let top = 0.0;
        let bottom = 1.0;
        let left = 0.0;
        let right = 1.0;
        self.add_vertices(
            &[
                ShapeVertex {
                    pos: top_left,
                    uv: Vec2::new(left, top),
                },
                ShapeVertex {
                    pos: top_right,
                    uv: Vec2::new(right, top),
                },
                ShapeVertex {
                    pos: bottom_left,
                    uv: Vec2::new(left, bottom),
                },
                ShapeVertex {
                    pos: bottom_right,
                    uv: Vec2::new(right, bottom),
                },
            ],
            &[0, 1, 2, 1, 2, 3],
        );
    }

    pub fn add_vertices(&mut self, vertices: &[ShapeVertex], indices: &[u32]) {
        let tex = self.texture.get_index();
        let fill_color = self.fill_color;
        let base_index = self.vertices.len() as u32;
        self.vertices.extend(vertices.iter().map(|shape_vertex| {
            BindlessVertex {
                pos: [shape_vertex.pos.x, shape_vertex.pos.y],
                uv: shape_vertex.uv.into(),
                color: fill_color,
                tex,
                ..Default::default()
            }
        }));
        self.indices
            .extend(indices.iter().map(|index| index + base_index));
    }
}

// Private API
// -----------

impl G2D {
    pub(crate) fn get_vertices(&self) -> &[BindlessVertex] {
        &self.vertices
    }

    pub(crate) fn get_indices(&self) -> &[u32] {
        &self.indices
    }

    pub(crate) fn reset_vertices(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}
