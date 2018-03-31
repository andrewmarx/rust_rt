use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f {x, y, z}
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn length_sq(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
    }

    pub fn cross(&self, v: &Vec3f) -> Vec3f {
        Vec3f::new(self.y*v.z - v.y*self.z, self.z*v.x - v.z*self.x, self.x*v.y -v.x*self.y)
    }

    pub fn perpendicular(&self) -> Vec3f {

        let mut v: Vec3f;

        if self.x.abs() < self.y.abs() {
            if self.x.abs() < self.z.abs() {
                v = Vec3f::new(1.0, 0.0, 0.0)
            } else {
                v = Vec3f::new(0.0, 0.0, 1.0)
            }
        } else {
            if self.y.abs() < self.z.abs() {
                v = Vec3f::new(1.0, 0.0, 0.0)
            } else {
                v = Vec3f::new(0.0, 0.0, 1.0)
            }
        }
        v = v.cross(self);
        v.normalize();
        v
    }

    pub fn dot(&self, v: Vec3f) -> f32 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }
}


impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}

impl ops::Add<f32> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x+rhs, self.y+rhs, self.z+rhs)
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z)
    }
}

impl ops::Sub<f32> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x-rhs, self.y-rhs, self.z-rhs)
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}

impl ops::Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self*rhs.x, self*rhs.y, self*rhs.z)
    }
}

impl ops::Div<f32> for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x/rhs, self.y/rhs, self.z/rhs)
    }
}