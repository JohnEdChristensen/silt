use glam::Vec3;

use crate::Vertex;

const H_W_COUNT: usize = 15;
const H_H_COUNT: usize = 15;

pub type Point = Vec3;
pub struct HeightMap(Vec<Vec<Vec3>>);
pub type Tri = (Vertex, Vertex, Vertex);
fn gaussian(x: f32, y: f32, a: f32, b: f32) -> f32 {
    a * (-(((x * x) / (2. * b * b)) + (y * y) / (2. * b * b))).exp()
}

pub fn create_gaussian(width: f32, height: f32, a: f32, b: f32) -> HeightMap {
    let mut h_map = vec![vec![Vec3::ZERO; H_W_COUNT + 1]; H_H_COUNT + 1];

    let cell_width = width / H_W_COUNT as f32;
    let cell_height = height / H_H_COUNT as f32;
    let z_step = cell_width / 4.;
    //let dw = cell_width;

    for (i, row) in h_map.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let x = i as f32 * cell_width - (width / 2.);
            let y = j as f32 * cell_height - (height / 2.);
            println!("{} {}", x, y);
            let z = gaussian(x, y, a, b);

            let z_steps = (z / z_step).floor() * z_step;

            //let dx = gaussian(x - dw, y, a, b) - gaussian(x + dw, y, a, b);
            //let dx_steps = (dx / z_step).floor() * z_step;
            //let dy = gaussian(x, y - dw, a, b) - gaussian(x, y + dw, a, b);
            //let dy_steps = (dy / z_step).floor() * z_step;

            //let normal = Vec3::new(2. * dw, 0., dx_steps)
            //    .normalize()
            //    .cross(Vec3::new(0., 2. * dw, dy_steps).normalize())
            //    .normalize();
            *val = Vec3::new(x, z_steps, y);
            //normal: [normal[0], normal[1], normal[2]],
            ////normal: [0., 0., 1.],
            //tex_coords: [x, y],
        }
    }
    HeightMap(h_map)
}
impl HeightMap {
    pub fn create_triangles(&self) -> (Vec<Vertex>, Vec<u16>) {
        let pos: Vec<_> = self.0.clone().into_iter().flatten().collect();
        let width = self.0[0].len();
        let height = self.0.len();
        let verts: Vec<_> = self
            .0
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
                        let i3 = (i + (j + 1) * width) as u16;
                        let i4 = (i + 1 + (j + 1) * width) as u16;

                        let p1 = pos[i1 as usize];
                        let p2 = pos[i2 as usize];
                        let p3 = pos[i3 as usize];
                        let p4 = pos[i4 as usize];
                        //let pm = (p1 + p2 + p3 + p4) / 4.;

                        //tris: [p1, p2, p3], [p2, p4, p3]
                        let t1_normal = (p2 - p1).cross(p3 - p1).normalize().into();
                        let t2_normal = (p4 - p2).cross(p3 - p2).normalize().into();

                        let v1 = Vertex {
                            position: p1.into(),
                            normal: t1_normal,
                            tex_coords: [p1.x, p1.y],
                        };
                        let v2 = Vertex {
                            position: p2.into(),
                            normal: t1_normal,
                            tex_coords: [p2.x, p2.y],
                        };
                        let v3 = Vertex {
                            position: p3.into(),
                            normal: t1_normal,
                            tex_coords: [p3.x, p3.y],
                        };
                        let v4 = Vertex {
                            position: p2.into(),
                            normal: t2_normal,
                            tex_coords: [p1.x, p1.y],
                        };
                        let v5 = Vertex {
                            position: p4.into(),
                            normal: t2_normal,
                            tex_coords: [p2.x, p2.y],
                        };
                        let v6 = Vertex {
                            position: p3.into(),
                            normal: t2_normal,
                            tex_coords: [p3.x, p3.y],
                        };

                        vec![v1, v2, v3, v4, v5, v6]
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let indices: Vec<_> = (0..verts.len()).map(|i| i as u16).collect();
        //// give each index a normal
        //let verts = indices
        //    .chunks(3)
        //    .flat_map(|is| {
        //        let p1 = pos[is[0] as usize];
        //        let p2 = pos[is[1] as usize];
        //        let p3 = pos[is[2] as usize];
        //        let p4 = pos[is[2] as usize];
        //        let t1_normal = (p2 - p1).cross(p3 - p1).normalize().into();
        //        let t2_normal = (p2 - p1).cross(p3 - p1).normalize().into();
        //        v1 = Vertex {
        //            position: p1.into(),
        //            normal,
        //            tex_coords: [p1.x, p1.y],
        //        };
        //        v2 = Vertex {
        //            position: p2.into(),
        //            normal,
        //            tex_coords: [p2.x, p2.y],
        //        };
        //        v3 = Vertex {
        //            position: p3.into(),
        //            normal,
        //            tex_coords: [p3.x, p3.y],
        //        };
        //        vec![v1]
        //    })
        //    .collect();
        (verts, indices)
    }
}
