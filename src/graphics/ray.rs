pub use math::Vec3f;

#[derive(Default, Clone)]
pub struct Ray {
    pub pos: Vec3f,
    pub dir: Vec3f,
}

impl Ray {
    pub fn new(p: &Vec3f, d: &Vec3f) -> Self {
        Ray {
            pos: p.clone(),
            dir: d.clone(),
        }
    }
}