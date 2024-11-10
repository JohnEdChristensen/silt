use glam::{Vec2, Vec3};

use crate::height_map::{create_gaussian, map_gen, HeightMap, Resolution};

pub struct World {
    pub height_map: HeightMap,
    pub ground_plane: HeightMap,
}
impl World {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            height_map: map_gen(
                Resolution {
                    x: 50,
                    y: 50,
                    z: 300,
                },
                Vec3 {
                    x: 600.,
                    y: 600.,
                    z: 200.,
                },
                Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                },
                Vec2 { x: 0., y: 0. },
            ), //create_gaussian((30, 30), width as f32, height as f32, 300., 300., 0.),
            ground_plane: map_gen(
                Resolution { x: 10, y: 10, z: 3 },
                Vec3 {
                    x: 300000.,
                    y: 300000.,
                    z: 100.,
                },
                Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                },
                Vec2 { x: 0., y: 0. },
            ),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(10., 10.)
    }
}
