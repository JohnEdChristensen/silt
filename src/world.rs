use crate::height_map::{create_gaussian, HeightMap};

pub struct World {
    pub height_map: HeightMap,
    pub ground_plane: HeightMap,
}
impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            height_map: create_gaussian((30, 30), width, height, 300., 300., 0.),
            ground_plane: create_gaussian((2, 2), width * 10., height * 10., 0., 1., -10.),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(4000., 4000.)
    }
}
