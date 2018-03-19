extern crate nalgebra as na;

use std::fs;
use std::fs::File;
use std::io::Write;
use na::{Vector3,};

fn main() {
    let nx = 200;
    let ny = 100;

    save_image(nx, ny);
}

fn save_image(x: i32, y: i32) {
    let output = String::from("P3\n") + &x.to_string() + " " + &y.to_string() + "\n255\n";

    fs::create_dir_all("./out/").expect("Could not create output directory.");
    let mut f = File::create("./out/test.ppm").expect("Unable to create file");

    f.write(output.as_bytes()).expect("Could not write file");

    for j in {0..y}.rev() {
        for i in 0..x {
            let color = Vector3::new(i as f32 / x as f32, j as f32 / y as f32, 0.2) * 255.99;

            let mut output = String::new();
            let mut sep = "";

            for val in color.iter() {
                let v = *val as i32;
                output.push_str(sep);
                output.push_str(&v.to_string());
                sep = " ";
            }
            output.push_str("\n");

            f.write(output.as_bytes()).expect("Could not write file");
        }
    }
}

struct Ray {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    fn point_at_param(&self, t: f32) -> Vector3<f32> {
        self.pos + t * self.dir
    }
}