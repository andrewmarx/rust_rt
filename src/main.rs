extern crate nalgebra as na;

use std::fs;
use std::fs::File;
use std::io::Write;
use na::{Vector3,};

fn main() {
    let width: i32 = 200;
    let height: i32 = 100;
    let mut frame: Vec<i32> = vec![0; (width * height * 3) as usize];

    for j in {0..height}.rev() {
        for i in 0..width {
            let color = Vector3::new(i as f32 / width as f32, j as f32 / height as f32, 0.2) * 255.99;

            let index: usize = ((width * j + i) * 3) as usize;

            frame[index] = color[0] as i32;
            frame[index + 1] = color[1] as i32;
            frame[index + 2] = color[2] as i32;
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

struct Ray {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    fn point_at_param(&self, t: f32) -> Vector3<f32> {
        self.pos + t * self.dir
    }
}