use crate::height_map::{create_gaussian, HeightMap};

pub struct World {
    pub height_map: HeightMap,
}
impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            height_map: create_gaussian(width, height, 100., 100.),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(2000., 2000.)
    }
}
