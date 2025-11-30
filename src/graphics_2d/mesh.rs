use nalgebra::Vector2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

/// A Mesh is the minimal unit of rendering.
///
/// Data is streamed from the CPU to the GPU each frame from each Mesh.
pub trait Mesh {
    fn vertices(&self) -> &[Vertex];
}

/// The GeometryMesh supports constructing procedural geometry, things like
/// lines, circles, and triangles.
pub struct GeometryMesh {
    vertices: Vec<Vertex>,
}

impl Mesh for GeometryMesh {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
}

impl GeometryMesh {
    /// Creates a new empty Mesh with pre-allocated internal memory for
    /// vertex data.
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(initial_capacity),
        }
    }

    /// Clears all geometry from the Mesh while retaining any allocated memory.
    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    /// Adds a triangle to the mesh.
    pub fn triangle(
        &mut self,
        p1: Vector2<f32>,
        p2: Vector2<f32>,
        p3: Vector2<f32>,
    ) {
        let color = [1.0, 1.0, 1.0, 1.0];
        self.vertices.extend_from_slice(&[
            Vertex {
                pos: p1.data.0[0],
                uv: [0.0, 0.0],
                color,
            },
            Vertex {
                pos: p2.data.0[0],
                uv: [0.0, 0.0],
                color,
            },
            Vertex {
                pos: p3.data.0[0],
                uv: [0.0, 0.0],
                color,
            },
        ]);
    }
}
