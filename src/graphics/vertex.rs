pub use math::Vec3f;

#[derive(Default, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3f,
    pub tx: f32,
    pub ty: f32,
    pub sx: i32,
    pub sy: i32
}

impl Vertex {
    pub fn new(v: &Vec3f) -> Self {
        Vertex {
            pos: v.clone(),
            tx: 0.0,
            ty: 0.0,
            sx: 0,
            sy: 0,
        }
    }
}

