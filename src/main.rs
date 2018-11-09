extern crate image;
extern crate rand;

//mod math;
mod graphics;
mod math;
//pub use math::Vec3f;
pub use graphics::geometry::*;
pub use graphics::camera::*;
pub use graphics::scene::*;

use rand::Rng;
use std::time::Instant;


fn main() {
    let mut scene = create_lamp_scene(5);

    let mut cam = Camera::new(1920,1080, 90.0);

    let start = Instant::now();
    cam.render(&mut scene);
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    cam.imgbuf.save("./out/output.png").unwrap();
}

fn create_lamp_scene(size: isize) -> Scene {
    let mut lamp = load_obj("./dat/lamp_hi.obj");
    lamp.calc_bounding_rad();

    let mut mat_red = Material::new();
    mat_red.set_color(Vec3f::new(1.0, 0.0, 0.0));

    let mut mat_green = Material::new();
    mat_green.set_color(Vec3f::new(0.0, 1.0, 0.0));

    let mut mat_blue = Material::new();
    mat_blue.set_color(Vec3f::new(0.0, 0.0, 1.0));

    let mat_vec = vec![mat_red, mat_green, mat_blue];

    let mut scene = Scene::new();

    for i in (-size)..(size+1) {
        for k in 0..(size*2 + 1) {
            let rand_num = rand::thread_rng().gen_range(0, 3);

            lamp.set_position(Vec3f::new(7.5*i as f32, -2.5, 10.0 + (k as f32)*7.5));
            lamp.mat = mat_vec[rand_num];

            scene.render_list.push(lamp.clone());
        }
    }

    scene
}