use glam::Vec3;

use crate::Vertex;

pub type Point = Vec3;
pub struct HeightMap {
    resolution: (usize, usize),
    points: Vec<Vec<Vec3>>,
}

fn gaussian(x: f32, y: f32, a: f32, b: f32) -> f32 {
    a * (-(((x * x) / (2. * b * b)) + (y * y) / (2. * b * b))).exp()
}

pub fn create_gaussian(
    resolution: (usize, usize),
    width: f32,
    height: f32,
    a: f32,
    b: f32,
    z_offset: f32,
) -> HeightMap {
    let mut h_map = vec![vec![Vec3::ZERO; resolution.0 + 1]; resolution.1 + 1];

    let cell_width = width / resolution.0 as f32;
    let cell_height = height / resolution.1 as f32;
    let z_step = cell_width / 4.;

    for (i, row) in h_map.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let x = i as f32 * cell_width - (width / 2.);
            let y = j as f32 * cell_height - (height / 2.);

            let z = gaussian(x, y, a, b);
            let z_steps = (z / z_step).floor() * z_step;

            *val = Vec3::new(x, z_steps + z_offset, y);
        }
    }
    HeightMap {
        points: h_map,
        resolution,
    }
}
impl HeightMap {
    pub fn create_triangles(&self) -> (Vec<Vertex>, Vec<u16>) {
        let pos: Vec<_> = self.points.clone().into_iter().flatten().collect();
        let width = self.resolution.0 + 1;
        let height = self.resolution.1 + 1;
        let verts: Vec<_> = self
            .points
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .flat_map(|(j, _cols)| -> Vec<Vertex> {
                        if i == width - 1 || j == height - 1 {
                            return vec![];
                        }
                        let i1 = (i + j * width) as u16;
                        let i2 = (i + 1 + j * width) as u16;
                        let i3 = (i + 1 + (j + 1) * width) as u16;
                        let i4 = (i + (j + 1) * width) as u16;

                        let p1 = pos[i1 as usize];
                        let p2 = pos[i2 as usize];
                        let p3 = pos[i3 as usize];
                        let p4 = pos[i4 as usize];
                        let p_middle = (p1 + p2 + p3 + p4) / 4.;

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
