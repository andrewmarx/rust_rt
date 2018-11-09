pub use math::vector::Vec3f;
pub use graphics::ray::*;
pub use graphics::intersection::*;

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

    pub fn set_color(&mut self, c: Vec3f) {
        self.color.x = c.x;
        self.color.y = c.y;
        self.color.z = c.z;
    }

    pub fn color(&self, r: &Ray, i: &Intersection) -> Vec3f {

        let light_pos = Vec3f::new(0.0, 0.0, 0.0);
        let mut diffuse_light;
        let position = r.pos + (r.dir*i.dist);

        diffuse_light = light_pos - position;
        diffuse_light.normalize();
        let mut diff = i.norm.dot(&diffuse_light).max(0.0);
        diff = (diff + 0.15).min(0.99);

        Vec3f::new(diff*self.color.x, diff*self.color.y, diff*self.color.z)
    }
}