use egui::Context;
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
    map_offset: Vec2,
    map_scale: Vec3,
    /// 1-1-1 to start
    scale: Vec3,
    /// points in the range x,y,z: 0.0-1.0
    normalized_points: Vec<Vec<f32>>,
    /// flag to determine if the vertex/index buffer needs to be updated
    pub dirty_points: bool,
}

impl HeightMap {
    pub fn new(
        map_offset: Vec2,
        map_scale: Vec3,
        model_scale: Vec3,
        resolution: Resolution,
    ) -> Self {
        let normalized_points = HeightMap::map_gen(&resolution, &map_scale, &map_offset);
        Self {
            resolution,
            map_offset,
            map_scale,
            scale: model_scale.xzy(),
            normalized_points,
            dirty_points: true,
        }
    }
    pub fn create_triangles(&mut self) -> (Vec<Vertex>, Vec<u32>) {
        assert!(self.dirty_points, "Unecessary creation of triangles!");
        self.normalized_points =
            HeightMap::map_gen(&self.resolution, &self.map_scale, &self.map_offset);

        let pos: Vec<_> = self
            .normalized_points
            .clone()
            .into_iter()
            .flatten()
            .collect();

        let verts: Vec<_> = self
            .normalized_points
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
        let indices: Vec<_> = (0..verts.len()).map(|i| i as u32).collect();
        (verts, indices)
    }

    pub fn gui(&mut self, ui: &Context) {
        egui::Window::new("")
            // .vscroll(true)
            .default_open(false)
            .max_width(1000.0)
            .max_height(800.0)
            .default_width(800.0)
            .resizable(false)
            .show(ui, |ui| {
                if ui.add(egui::Button::new("Click me")).clicked() {
                    self.scale *= 1.1;
                    self.dirty_points = true;
                    println!("PRESSED")
                }

                if ui
                    .add(egui::Slider::new(&mut self.scale.x, 0.0..=10000.0).text("h size"))
                    .changed()
                {
                    self.scale.z = self.scale.x;
                    self.dirty_points = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.scale.y, 0.0..=1000.0).text("v size"))
                    .changed()
                {
                    self.dirty_points = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.map_offset.x, -1.0..=1.0).text("map_x"))
                    .changed()
                {
                    self.dirty_points = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.map_offset.y, -1.0..=1.0).text("map_y"))
                    .changed()
                {
                    self.dirty_points = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.resolution.x, 2..=400).text("map_y"))
                    .changed()
                {
                    self.resolution.y = self.resolution.x;
                    self.dirty_points = true;
                }
            });
    }

    pub fn map_gen(resolution: &Resolution, map_scale: &Vec3, map_offset: &Vec2) -> Vec<Vec<f32>> {
        let map_left = 0.0 + map_offset.x as f64;
        let map_top = 0.0 + map_offset.y as f64;
        let map_right = (1.0 * map_scale.x + map_offset.x) as f64;
        let map_bottom = (1.0 * map_scale.y + map_offset.y) as f64;

        let fbm = Fbm::<Perlin>::default();
        PlaneMapBuilder::new(fbm)
            .set_size(resolution.x, resolution.y)
            .set_x_bounds(map_left, map_right)
            .set_y_bounds(map_top, map_bottom)
            .build()
            .into_iter()
            .map(|v| v as f32)
            .chunks(resolution.x)
            .into_iter()
            .map(|c| c.collect())
            .collect()
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
//
