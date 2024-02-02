#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub rgba: [f32; 4],
    pub pos: [f32; 4],
}

impl Vertex {
    pub fn new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        Vertex {
            rgba: [r, g, b, a],
            pos: [x, y, 0.0, 1.0],
        }
    }
}
