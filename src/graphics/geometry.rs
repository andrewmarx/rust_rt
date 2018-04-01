
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub use graphics::vertex::*;
pub use graphics::material::*;

#[derive(Default, Clone)]
pub struct Geom {
    pub pos: Vec3f,
    pub dir: Vec3f,
    pub up: Vec3f,

    pub base_vertex_array: Vec<Vec3f>,
    pub vertex_array: Vec<Vertex>,
    pub face_array: Vec<usize>,

    pub rad: f32,
    pub cam_dist: f32,

    pub mat: Material
}

impl Geom {
    pub fn set_position(&mut self, v: Vec3f) {
        self.pos = v;
    }

    pub fn translate(&mut self, v: Vec3f) {
        self.pos = self.pos + v;
    }

    pub fn calc_bounding_rad(&mut self) {
        let mut d;

        for v in self.base_vertex_array.iter() {
            d = (*v-self.pos).length();
            self.rad = self.rad.max(d);
        }

        //self.vertex_array = (0..self.base_vertex_array.count()).map(Vec3f).collect(); // Efficiently initialize vec for expected number of elements
        self.vertex_array = Vec::new();

        for v in self.base_vertex_array.iter() {
            self.vertex_array.push(Vertex::new(&(*v+self.pos)));
        }
    }

    pub fn transform(&mut self) {
        for i in 0..(self.base_vertex_array.iter().count()) {
            self.vertex_array[i].pos.set(&(self.pos + self.base_vertex_array[i]));
        }
    }

    pub fn scale(&self) {

    }
}

pub fn load_obj(filepath: &'static str) -> Geom {
    let mut geom = Geom {..Default::default()};

    let f = File::open(filepath).expect("file not found");

    let reader = BufReader::new(f);
    let lines = reader.lines();
    // lines is a instance of some type which implements Iterator<Item=&str>

    for line in lines {
        let l = line.unwrap();

        let res = l.split_whitespace().collect::<Vec<_>>();

        if res.iter().count() < 1 {
            continue;
        }

        //println!("{}", res[0]);
        match res[0].trim() {
            "v" => geom.base_vertex_array.push(Vec3f::new(res[1].parse().unwrap(),res[2].parse().unwrap(),res[3].parse().unwrap())),
            "f" => {
                for i in 0..(res.iter().count()-3){
                    geom.face_array.push(res[1].parse::<usize>().unwrap()-1);
                    geom.face_array.push(res[2+i].parse::<usize>().unwrap()-1);
                    geom.face_array.push(res[3+i].parse::<usize>().unwrap()-1);
                }
            },
            _ => {},
        }
    }
    println!("Loaded object. Vertices: {}. Faces: {}.", geom.base_vertex_array.iter().count(), geom.face_array.iter().count());
    geom
}