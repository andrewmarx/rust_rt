use graphics::geometry::Geom;

pub struct Scene {
    pub render_list: Vec<Geom>,
    //light_list: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            render_list: Vec::new(),
        }
    }
}