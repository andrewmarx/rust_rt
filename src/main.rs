extern crate image;

mod math;
mod graphics;

use std::fs;
use std::fs::File;
use std::io::Write;

pub use math::Vec3f;
pub use graphics::geometry::*;

fn main() {
    let width: i32 = 400;
    let height: i32 = 200;
    let mut frame: Vec<i32> = vec![0; (width * height * 3) as usize];
    let g: Geom = load_obj("./dat/lamp.obj");
    let lower_left_corner = Vec3f::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3f::new(4.0, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, 2.0, 0.0);
    let origin = Vec3f::new(0.0, 0.0, 0.0);

    for j in {0..height}.rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;

            let r = Ray {
                pos: origin,
                dir: lower_left_corner + horizontal*u + vertical*v,
            };

            let col = color(&r) * 255.99;

            let index: usize = ((width * j + i) * 3) as usize;

            frame[index] = col.x as i32;
            frame[index + 1] = col.y as i32;
            frame[index + 2] = col.z as i32;
        }
    }

    save_image(frame.as_slice(),width, height);
}

fn save_image(fr: &[i32], x: i32, y: i32) {
    let output = String::from("P3\n") + &x.to_string() + " " + &y.to_string() + "\n255\n";

    fs::create_dir_all("./out/").expect("Could not create output directory.");
    let mut f = File::create("./out/test2.ppm").expect("Unable to create file");

    f.write(output.as_bytes()).expect("Could not write file");

    //TODO: Simplify to single loop
    for j in {0..y}.rev() {
        for i in 0..x {
            let index: usize = ((x * j + i) * 3) as usize;

            //println!("{}", f[0]);
            let output = (&fr[index]).to_string() + " " + &(&fr[index + 1]).to_string() + " " + &(&fr[index + 2]).to_string() + "\n";

            f.write(output.as_bytes()).expect("Could not write file");
        }
    }
}

fn hit_sphere(center: &Vec3f, radius: f32, r: &Ray) -> f32 {
    let oc = r.pos - *center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 *oc.dot(r.dir);
    let c = oc.dot(oc) - radius*radius;
    let disc = b*b - 4.0*a*c;

    if disc < 0.0 {
        -1.0
    } else {
        (-b - disc.sqrt()) / (2.0*a)
    }
}

fn color(r: &Ray) -> Vec3f {
    let t = hit_sphere(&Vec3f::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let mut n = r.point_at_param(t) - Vec3f::new(0.0, 0.0, -1.0);
        n.normalize();
        return 0.5*Vec3f::new(n.x+1.0, n.y+1.0, n.z+1.0);
    }

    let mut unit_direction: Vec3f = r.dir;
    unit_direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);

    (1.0-t)*Vec3f::new(1.0,1.0,1.0) + t*Vec3f::new(0.5,0.7,1.0)
}

struct Ray {
    pos: Vec3f,
    dir: Vec3f,
}

impl Ray {
    fn point_at_param(&self, t: f32) -> Vec3f {
        self.pos + t * self.dir
    }
}