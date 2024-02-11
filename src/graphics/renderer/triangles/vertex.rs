#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub rgba: [f32; 4],
    pub pos: [f32; 4],
}

impl Vertex {
    pub fn new(pos: [f32; 2], vel: [f32; 2], rgba: [f32; 4]) -> Self {
        let [x, y] = pos;
        let [vx, vy] = vel;
        Vertex {
            rgba,
            pos: [x, y, vx, vy],
        }
    }
}
