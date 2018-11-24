use image::{RgbaImage};
pub use math::vector::Vec3f;
pub use graphics::intersection::Intersection;
pub use graphics::geometry::Geom;
pub use graphics::scene::Scene;
pub use graphics::ray::Ray;
pub use graphics::vertex::Vertex;

pub struct Camera {
    pos: Vec3f,
    dir: Vec3f,
    up: Vec3f,

    right: Vec3f,
    bottom_left: Vec3f,

    width: i32,
    height: i32,
    fov: f32,
    focal_length: f32,

    ray_list: Vec<Vec3f>,
    intersection_list: Vec<Intersection>,

    lookup_x: Vec<f32>,
    lookup_y: Vec<f32>,

    pub imgbuf: RgbaImage,

    left_cull: Vec3f,
    right_cull: Vec3f,
    top_cull: Vec3f,
    bottom_cull: Vec3f,
}

impl Camera {
    pub fn new(width: i32, height: i32, mut fov: f32) -> Camera {
        fov = if fov > 180.0 {
            180.0
        } else if fov < 10.0 {
            10.0
        } else {
            fov
        };

        let focal_length = (width as f32/2.0) / (fov/2.0).to_radians().tan();

        let pos = Vec3f::new(0.0, 0.0, 0.0);
        let dir = Vec3f::new(0.0, 0.0, 1.0);
        let up = Vec3f::new(0.0, 1.0, 0.0);

        let right = up.cross(&dir);

        let bottom_left = pos +
            (focal_length*dir) +
            (up*(-height as f32/2.0)) +
            (right*(-width as f32/2.0));

        Camera {
            pos,
            dir,
            up,
            right,
            bottom_left,
            width,
            height,
            fov,
            focal_length,
            ray_list: vec![Vec3f::new(0.0, 0.0, 0.0); (width * height) as usize],
            intersection_list: vec![Intersection::new(); (width * height) as usize],
            lookup_x: vec![0.0; width as usize + 1],
            lookup_y: vec![0.0; height as usize + 1],
            imgbuf: RgbaImage::new(width as u32, height as u32),
            left_cull: Vec3f::new(0.0, 0.0, 0.0),
            right_cull: Vec3f::new(0.0, 0.0, 0.0),
            top_cull: Vec3f::new(0.0, 0.0, 0.0),
            bottom_cull: Vec3f::new(0.0, 0.0, 0.0),
        }
    }

    fn generate_cull_planes(&mut self) {
        let mut index;

        for y in 0..self.height {
            for x in 0..self.width {
                index = y*self.width + x;

                let mut screen_point = self.bottom_left +
                    (self.up * (y as f32+0.5)) +
                    (self.right * (x as f32+0.5)) -
                    self.pos;

                screen_point.normalize();

                self.ray_list[index as usize].set(&screen_point);
            }
        }

        self.right_cull = self.up.cross(&(self.bottom_left + self.right*self.width as f32));
        self.right_cull.normalize();

        self.left_cull = self.up.cross(&self.bottom_left);
        self.left_cull.normalize();

        self.bottom_cull = self.bottom_left.cross(&self.right);
        self.bottom_cull.normalize();

        self.top_cull = (self.bottom_left + (self.up*self.height as f32)).cross(&self.right);
        self.top_cull.normalize();

        let mut temp_vec;
        for x in 0..(self.width+1) {
            temp_vec = self.bottom_left +
                (self.up * (self.height as f32 / 2.0)) +
                (self.right * x as f32) -
                self.pos;

            temp_vec.normalize();

            self.lookup_x[x as usize] = self.right.dot(&temp_vec);
        }

        for y in 0..(self.height+1) {
            temp_vec = self.bottom_left +
                (self.up * y as f32) +
                (self.right * (self.width as f32 / 2.0)) -
                self.pos;

            temp_vec.normalize();

            self.lookup_y[y as usize] = self.up.dot(&temp_vec);
        }
    }

    fn basic_frustrum_cull(&self, scene_list: &mut Vec<Geom>) -> Vec<Geom> {
        let mut render_list: Vec<Geom> = Vec::new();
        let mut d;
        let mut camspherevec;

        for mesh in scene_list.iter_mut() {
            camspherevec = mesh.pos - self.pos;

            d = -self.dir.dot(&camspherevec);
            if d > mesh.rad {
                continue;
            }

            d = self.right_cull.dot(&camspherevec);
            if d > mesh.rad {
                continue;
            }

            d = self.left_cull.dot(&camspherevec);
            if d < -mesh.rad {
                continue;
            }

            d = self.bottom_cull.dot(&camspherevec);
            if d < -mesh.rad {
                continue;
            }

            d = self.top_cull.dot(&camspherevec);
            if d > mesh.rad {
                continue;
            }

            mesh.transform();
            mesh.cam_dist = camspherevec.length() - mesh.rad;
            render_list.push(mesh.clone());
        }

        render_list.sort_by(|a, b| a.cam_dist.partial_cmp(&b.cam_dist).unwrap());
        render_list
    }

    pub fn sort_verts(&self, render_list: &mut Vec<Geom>) {
        let mut xd: f32;
        let mut yd: f32;
        let mut dist: f32;

        let mut low: i32;
        let mut mid: i32;
        let mut high: i32;

        let mut av;

        let lookup_x_min = self.lookup_x[0];
        let lookup_x_max = self.lookup_x[self.width as usize];
        let lookup_y_min = self.lookup_y[0];
        let lookup_y_max = self.lookup_y[self.height as usize];

        let half_width = self.width/2;
        let half_height = self.height/2;

        for geom in render_list.iter_mut() {
            for v in geom.vertex_array.iter_mut() {
                av = v.pos - self.pos;
                dist = av.dot(&self.up);
                av = av - (self.up * dist);
                av.normalize();
                xd = self.right.dot(&av);

                av = v.pos - self.pos;
                dist = av.dot(&self.right);
                av = av - (self.right * dist);
                av.normalize();
                yd = self.up.dot(&av);

                if xd >= lookup_x_max {
                    v.sx = self.width-1;
                } else if xd < lookup_x_min {
                    v.sx = 0;
                } else {
                    low = 0;
                    high = self.width;
                    mid = half_width;

                    while (high-low) > 1 {
                        if self.lookup_x[mid as usize] < xd {
                            low = mid;
                        } else {
                            high = mid;
                        }
                        mid = (low+high)/2;
                    }

                    v.sx = low;
                }

                if yd >= lookup_y_max {
                    v.sy = self.height - 1;
                } else if yd < lookup_y_min {
                    v.sy = 0;
                } else {
                    low = 0;
                    high = self.height;
                    mid = half_height;

                    while (high-low) > 1 {
                        if self.lookup_y[mid as usize] < yd {
                            low=mid;
                        } else {
                            high=mid;
                        }

                        mid = (low + high)/2;
                    }

                    v.sy = low;
                }
            }
        }
    }

    pub fn render(&mut self, scene: &mut Scene) {
        self.generate_cull_planes();

        let mut render_list = self.basic_frustrum_cull(&mut scene.render_list);

        self.sort_verts(&mut render_list);

        for geom in render_list {
            self.render_tris(&geom);
        }

        self.render_material();
    }

    pub fn render_tris(&mut self, geom: &Geom) {
        //println!("render_tris()");
        let mut v0: &Vertex;
        let mut v1: &Vertex;
        let mut v2: &Vertex;

        let mut minx: i32;
        let mut maxx: i32;
        let mut miny: i32;
        let mut maxy: i32;

        let mut norm = Vec3f::new(0.0, 0.0, 0.0);

        let mut ray = Ray::new(&self.pos, &Vec3f::new(0.0, 0.0, 0.0));

        //Moller-Trumbore intersection
        let mut p = Vec3f::new(0.0, 0.0, 0.0);
        let mut q: Vec3f;
        let mut t: Vec3f;

        let mut det: f32;
        let mut inv_det: f32;
        let mut u: f32;
        let mut v: f32;
        let mut t2: f32;

        let mut e1: Vec3f;
        let mut e2: Vec3f;

        let array_size = geom.face_array.iter().count();
        let mut i: usize = 0;

        let mut flag;
        //println!("array_size: {}", array_size);
        while i < array_size {
            flag = false;

            v0 = &geom.vertex_array[geom.face_array[i]];
            v1 = &geom.vertex_array[geom.face_array[i + 1]];
            v2 = &geom.vertex_array[geom.face_array[i + 2]];

            maxx = v0.sx.max(v1.sx.max(v2.sx));
            minx = v0.sx.min(v1.sx.min(v2.sx));
            maxy = v0.sy.max(v1.sy.max(v2.sy));
            miny = v0.sy.min(v1.sy.min(v2.sy));

            e1 = v1.pos - v0.pos;
            e2 = v2.pos - v0.pos;

            for y in miny..(maxy + 1) {
                for x in minx..(maxx + 1) {
            //for y in 0..(self.height) {
            //    for x in minx..(self.width) {
                    let index = (y*self.width + x) as usize;
                    let mut current_intersection = self.intersection_list.get_mut(index).unwrap();

                    if current_intersection.dist < (geom.cam_dist - geom.rad) {
                        continue;
                    }

                    ray.dir.set(&self.ray_list[index]);

                    p.set(&ray.dir.cross(&e2));

                    det = e1.dot(&p);

                    if det < -0.00001 {
                        continue;
                    }

                    inv_det = 1.0/det;

                    t = ray.pos - v0.pos;

                    u = t.dot(&p)*inv_det;

                    if u < -0.00001 || u > 1.00001 {
                        continue;
                    }

                    q = t.cross(&e1);

                    v = ray.dir.dot(&q) * inv_det;

                    if v < -0.00001 || (u + v) > 1.00001 {
                        continue;
                    }

                    t2 = e2.dot(&q) * inv_det;

                    if t2 > 0.00001 && t2 < current_intersection.dist {
                        if !flag {
                            flag = true;
                            norm.set(&e1.cross(&e2));
                            norm.normalize();
                        }

                        current_intersection.dist = t2;
                        current_intersection.u = u;
                        current_intersection.v = v;
                        current_intersection.norm.set(&norm);
                        current_intersection.mat = geom.mat;
                    }
                }
            }
            //println!("Tri {}",i/3);
            i += 3;
        }
    }

    pub fn render_material(&mut self) {
        let mut ray = Ray::new(&self.pos, &Vec3f::new(0.0, 0.0, 0.0));
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y*self.width + x) as usize;

                let mut current_intersection = self.intersection_list.get_mut(index).unwrap();
                ray.pos = *self.ray_list.get(index).unwrap();

                let color = current_intersection.mat.color(&ray, &current_intersection)*255.99;

                self.imgbuf.get_pixel_mut(x as u32, y as u32).data = [color.x as u8, color.y as u8, color.z as u8, 255];
            }
        }
    }
}

