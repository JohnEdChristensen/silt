use glam::Vec3;

use crate::Vertex;

const H_W_COUNT: usize = 80;
const H_H_COUNT: usize = 80;

pub type Point = Vec3;
pub struct HeightMap(Vec<Vec<Vertex>>);
pub type Tri = (Vertex, Vertex, Vertex);
fn gaussian(x: f32, y: f32, a: f32, b: f32) -> f32 {
    a * (-(((x * x) / (2. * b * b)) + (y * y) / (2. * b * b))).exp()
}

pub fn create_gaussian(width: f32, height: f32, a: f32, b: f32) -> HeightMap {
    let mut h_map = vec![
        vec![
            Vertex {
                position: [0., 0., 0.],
                normal: [0., 0., 0.],
                tex_coords: [0., 0.]
            };
            H_W_COUNT
        ];
        H_H_COUNT
    ];

    let cell_width = width / H_W_COUNT as f32;
    let cell_height = height / H_H_COUNT as f32;
    let dw = 0.01;

    for (i, row) in h_map.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let x = i as f32 * cell_width - (width / 2.);
            let y = j as f32 * cell_height - (height / 2.);
            let z = gaussian(x, y, a, b);
            let dx = gaussian(x - dw, y, a, b) - gaussian(x + dw, y, a, b);
            let dy = gaussian(x, y - dw, a, b) - gaussian(x, y + dw, a, b);
            let normal = Vec3::new(2. * dw, 0., dx)
                .normalize()
                .cross(Vec3::new(0., 2. * dw, dy).normalize())
                .normalize();
            *val = Vertex {
                position: [x, z, y],
                normal: [normal[0], normal[1], normal[2]],
                //normal: [0., 0., 1.],
                tex_coords: [x, y],
            }
        }
    }
    HeightMap(h_map)
}
impl HeightMap {
    pub fn create_triangles(&self) -> (Vec<Vertex>, Vec<u16>) {
        let verts = self.0.clone().into_iter().flatten().collect();
        let width = self.0[0].len();
        let height = self.0.len();
        let indices = self
            .0
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .flat_map(|(j, _cols)| -> Vec<u16> {
                        if i == width - 1 || j == height - 1 {
                            return vec![];
                        }
                        let p1 = (i + j * width) as u16;
                        let p2 = (i + 1 + j * width) as u16;
                        let p3 = (i + (j + 1) * width) as u16;
                        let p4 = (i + 1 + (j + 1) * width) as u16;
                        vec![p1, p2, p3, p2, p4, p3]
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        (verts, indices)
    }
}
