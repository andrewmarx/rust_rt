pub use math::Vec3f;

#[derive(Default, Clone, Copy)]
pub struct Material {
    color: Vec3f
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Vec3f::new(0.0,0.0,0.0)
        }
    }

    pub fn color(&self) -> Vec3f {
        self.color
    }
}