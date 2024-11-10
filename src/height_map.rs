use glam::{Vec2, Vec3, Vec3Swizzles};
use itertools::Itertools;
use noise::{utils::*, Fbm, Perlin};

use crate::Vertex;

pub struct Resolution {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub type Point = Vec3;
pub struct HeightMap {
    resolution: Resolution,
    /// 1-1-1 to start
    scale: Vec3,
    points: Vec<Vec<f32>>,
}

fn gaussian(x: f32, y: f32, a: f32, b: f32) -> f32 {
    a * (-(((x * x) / (2. * b * b)) + (y * y) / (2. * b * b))).exp()
}

impl HeightMap {
    pub fn create_triangles(&self) -> (Vec<Vertex>, Vec<u16>) {
        let pos: Vec<_> = self.points.clone().into_iter().flatten().collect();

        dbg!(self.points.len());
        dbg!(self.points[0].len());
        let verts: Vec<_> = self
            .points
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .flat_map(|(j, _cols)| -> Vec<Vertex> {
                        let x_count = self.resolution.x;
                        if i == x_count - 1 || j == self.resolution.y - 1 {
                            return vec![];
                        }

                        let i1 = i + j * x_count;
                        let i2 = i + 1 + j * x_count;
                        let i3 = i + 1 + (j + 1) * x_count;
                        let i4 = i + (j + 1) * x_count;
                        dbg!(i);
                        dbg!(j);
                        dbg!(i1);
                        dbg!(i2);
                        dbg!(i3);
                        dbg!(i4);
                        let x = (i as f32 / x_count as f32) - 0.5;
                        let y = (j as f32 / x_count as f32) - 0.5;
                        let xn = ((i + 1) as f32 / x_count as f32) - 0.5;
                        let yn = ((j + 1) as f32 / x_count as f32) - 0.5;

                        let p4 = Vec3 {
                            x,
                            z: y,
                            y: pos[i1],
                        } * self.scale;
                        let p3 = Vec3 {
                            x: xn,
                            z: y,
                            y: pos[i2],
                        } * self.scale;
                        let p2 = Vec3 {
                            x: xn,
                            z: yn,
                            y: pos[i3],
                        } * self.scale;
                        let p1 = Vec3 {
                            x,
                            z: yn,
                            y: pos[i4],
                        } * self.scale;

                        let p_middle = (p1 + p2 + p3 + p4) / 4.;

                        // make 4 tris, all sharing a center point
                        vec![
                            make_verts(p1, p2, p_middle),
                            make_verts(p2, p3, p_middle),
                            make_verts(p3, p4, p_middle),
                            make_verts(p4, p1, p_middle),
                        ]
                        .into_iter()
                        .flatten()
                        .collect()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let indices: Vec<_> = (0..verts.len()).map(|i| i as u16).collect();
        (verts, indices)
    }
}

pub fn map_gen(
    resolution: Resolution,
    model_scale: Vec3,
    map_scale: Vec3,
    map_offset: Vec2,
) -> HeightMap {
    let map_left = 0.0 + map_offset.x as f64;
    let map_top = 0.0 + map_offset.y as f64;
    let map_right = (1.0 * map_scale.x + map_offset.x) as f64;
    let map_bottom = (1.0 * map_scale.y + map_offset.y) as f64;

    let fbm = Fbm::<Perlin>::default();
    let points = PlaneMapBuilder::new(fbm)
        .set_size(resolution.x, resolution.y)
        .set_x_bounds(map_left, map_right)
        .set_y_bounds(map_top, map_bottom)
        .build()
        .into_iter()
        .map(|v| v as f32)
        .chunks(resolution.x)
        .into_iter()
        .map(|c| c.collect())
        .collect();

    HeightMap {
        points,
        resolution,
        scale: model_scale.xzy(),
    }
}
pub fn create_gaussian(
    resolution: Resolution,
    scale: Vec3,
    a: f32,
    b: f32,
    z_offset: f32,
) -> HeightMap {
    let mut h_map = vec![vec![0.0; resolution.x]; resolution.y];

    let z_step = a / resolution.z as f32;

    for (i, row) in h_map.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            //normalized between 0 and 1
            let x = i as f32 / resolution.x as f32;
            let y = j as f32 / resolution.y as f32;

            let z = gaussian(x, y, a, b);
            let z_steps = (z / z_step).floor() * z_step;

            *val = z_steps + z_offset;
        }
    }
    HeightMap {
        resolution,
        scale,
        points: h_map,
    }
}

fn make_verts(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec<Vertex> {
    let normal = (p2 - p1).cross(p3 - p1).normalize().into();
    vec![
        Vertex {
            position: p1.into(),
            normal,
            tex_coords: [p1.x, p1.y],
        },
        Vertex {
            position: p2.into(),
            normal,
            tex_coords: [p2.x, p2.y],
        },
        Vertex {
            position: p3.into(),
            normal,
            tex_coords: [p3.x, p3.y],
        },
    ]
}
//noise
//    .into_iter()
//    .map(|row| {
//        row.map(|v| match v {
//            x if x < -0.2 => TileType::DeepWater,
//            x if x < -0.1 => TileType::ShallowWater,
//            x if x < 0.0 => TileType::Beach,
//            x if x < 0.3 => TileType::TallGrass,
//            x if x < 0.4 => TileType::Hill,
//            x if x < 1.0 => TileType::Mountain,
//            _ => TileType::Empty,
//        })
//        .collect()
//    })
//    .collect()
