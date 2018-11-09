pub use math::vector::Vec3f;
pub use graphics::material::Material;

#[derive(Default, Clone, Copy)]
pub struct Intersection {
    pub dist: f32,
    pub u: f32,
    pub v: f32,
    pub norm: Vec3f,
    pub mat: Material,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            dist: 1000.0,
            u: 0.0,
            v: 0.0,
            norm: Vec3f::new(0.0, 0.0, 0.0),
            mat: Material::new()
        }
    }
}