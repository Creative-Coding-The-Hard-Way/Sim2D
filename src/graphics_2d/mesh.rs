use {
    crate::graphics_2d::material::Material, nalgebra::Vector2, std::sync::Arc,
};

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub texture_index: i32,
}

impl Vertex {
    pub fn new(
        pos: [f32; 2],
        uv: [f32; 2],
        color: [f32; 4],
        texture_index: i32,
    ) -> Self {
        Self {
            pos,
            uv,
            color,
            texture_index,
        }
    }
}

/// A Mesh is the minimal unit of rendering.
///
/// Data is streamed from the CPU to the GPU each frame from each Mesh.
pub trait Mesh {
    fn vertices(&self) -> &[Vertex];
    fn indices(&self) -> &[u32];
    fn material(&self) -> &Arc<Material>;
}

/// The GeometryMesh supports constructing procedural geometry, things like
/// lines, circles, and triangles.
pub struct GeometryMesh {
    color: [f32; 4],
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    material: Arc<Material>,
}

impl Mesh for GeometryMesh {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn material(&self) -> &Arc<Material> {
        &self.material
    }
}

impl GeometryMesh {
    /// Creates a new empty Mesh with pre-allocated internal memory for
    /// vertex data.
    pub fn new(initial_capacity: usize, material: Arc<Material>) -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            vertices: Vec::with_capacity(initial_capacity),
            indices: Vec::with_capacity(initial_capacity),
            material,
        }
    }

    /// Clears all geometry from the Mesh while retaining any allocated memory.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    /// Adds a triangle to the mesh.
    pub fn triangle(
        &mut self,
        p1: Vector2<f32>,
        p2: Vector2<f32>,
        p3: Vector2<f32>,
    ) {
        let base_index = self.vertices.len() as u32;

        self.vertices.extend_from_slice(&[
            Vertex::new(p1.data.0[0], [0.0, 0.0], self.color, -1),
            Vertex::new(p2.data.0[0], [0.0, 0.0], self.color, -1),
            Vertex::new(p3.data.0[0], [0.0, 0.0], self.color, -1),
        ]);
        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
        ]);
    }

    /// Adds an axis-aligned quad to the mesh.
    pub fn aligned_quad(
        &mut self,
        texture_index: i32,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) {
        let base_index = self.vertices.len() as u32;

        let left = center_x - width / 2.0;
        let right = center_x + width / 2.0;
        let top = center_y + height / 2.0;
        let bottom = center_y - height / 2.0;

        self.vertices.extend_from_slice(&[
            Vertex::new([left, top], [0.0, 0.0], self.color, texture_index),
            Vertex::new([left, bottom], [0.0, 1.0], self.color, texture_index),
            Vertex::new([right, bottom], [1.0, 1.0], self.color, texture_index),
            Vertex::new([right, top], [1.0, 0.0], self.color, texture_index),
        ]);
        self.indices.extend_from_slice(&[
            // triangle 1
            base_index,     // top left
            base_index + 1, // bottom left
            base_index + 2, // bottom right
            // triangle 2
            base_index,     // top left
            base_index + 2, // bottom right
            base_index + 3, // top right
        ]);
    }
}
