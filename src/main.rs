use std::fs;
use std::fs::File;
use std::io::Write;


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
            let r = i as f64 / x as f64;
            let g = j as f64 / y as f64;
            let b= 0.2;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            let output = ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";

            f.write(output.as_bytes()).expect("Could not write file");
        }
    }
}