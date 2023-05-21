use crate::graphics::vulkan_api::{BindlessVertex, TextureId};

pub struct G2D {
    vertices: Vec<BindlessVertex>,
    indices: Vec<u32>,
    pub clear_color: [f32; 4],
}

impl Default for G2D {
    fn default() -> Self {
        Self {
            vertices: Vec::with_capacity(10_000),
            indices: Vec::with_capacity(10_1000),
            clear_color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl G2D {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        texture_id: TextureId,
    ) {
        let top = 1.0;
        let bottom = 0.0;
        let left = 0.0;
        let right = 1.0;
        let tex = texture_id.get_index() as f32;
        let base_index = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&[
            BindlessVertex {
                pos: [x, y, 0.0, 1.0],
                uv: [left, top, tex],
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            BindlessVertex {
                pos: [x + w, y, 0.0, 1.0],
                uv: [right, top, tex],
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            BindlessVertex {
                pos: [x, y + h, 0.0, 1.0],
                uv: [left, bottom, tex],
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            BindlessVertex {
                pos: [x + w, y + h, 0.0, 1.0],
                uv: [right, bottom, tex],
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
        ]);
        self.indices.extend_from_slice(&[
            // Upper Triangle
            base_index,
            base_index + 1,
            base_index + 2,
            // Lower Triangle
            base_index + 1,
            base_index + 2,
            base_index + 3,
        ]);
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
    }
}
