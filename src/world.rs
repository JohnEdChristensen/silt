use glam::{Vec2, Vec3};

use crate::{
    height_map::{HeightMap, Resolution},
    Vertex,
};

pub struct World {
    pub height_map: HeightMap,
    pub ground_plane: HeightMap,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            height_map: HeightMap::new(
                Vec2 { x: 0., y: 0. },
                Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                },
                Vec3 {
                    x: 600.,
                    y: 600.,
                    z: 200.,
                },
                Resolution {
                    x: 50,
                    y: 50,
                    z: 300,
                },
            ),
            ground_plane: HeightMap::new(
                Vec2 { x: 0., y: 0. },
                Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                },
                Vec3 {
                    x: 6000.,
                    y: 6000.,
                    z: 200.,
                },
                Resolution {
                    x: 50,
                    y: 50,
                    z: 300,
                },
            ),
        }
    }

    pub fn update_geometry(&mut self) -> (Vec<Vertex>, Vec<u32>) {
        let (hv, hi) = self.height_map.create_triangles();
        let (gv, gi) = self.ground_plane.create_triangles();
        let gi_offset = gi.iter().map(|i| i + hi.len() as u32).collect();
        let verts = [hv, gv].concat();
        let indices = [hi, gi_offset].concat();
        (verts, indices)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(10., 10.)
    }
}
