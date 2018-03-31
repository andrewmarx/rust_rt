extern crate image;

mod math;
mod graphics;

use std::fs;
use std::fs::File;
use std::io::Write;

pub use math::Vec3f;
pub use graphics::geometry::*;
pub use graphics::camera::*;
pub use graphics::scene::*;

fn main() {
    let width: i32 = 400;
    let height: i32 = 200;
    let mut frame: Vec<i32> = vec![0; (width * height * 3) as usize];
    let g: Geom = load_obj("./dat/lamp.obj");

    let mut cam = Camera::new(1024,1024, 90.0);
    cam.render(&mut create_lamp_scene());
}

fn create_lamp_scene() -> Scene {
    Scene::new()
}