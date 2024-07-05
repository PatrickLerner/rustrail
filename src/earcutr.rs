//! This code in file is mostly borrowed from [bevy-earcutr](https://github.com/frewsxcv/bevy-earcutr)
//! With minor adjustments to make it work for our implementation.

use crate::mesh::MeshBuilder;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use geo::{coord, LineString, Polygon};
use std::convert::TryFrom;

type EarcutrIndices = Vec<usize>;
type EarcutrVertices = Vec<f64>;
type BevyIndices = Vec<u32>;
type BevyVertices = Vec<[f32; 3]>;

#[derive(Debug)]
pub struct EarcutrInput {
    pub vertices: EarcutrVertices,
    pub interior_indices: EarcutrIndices,
}

#[derive(Debug)]
pub struct EarcutrResult {
    pub vertices: EarcutrVertices,
    pub triangle_indices: EarcutrIndices,
}

impl EarcutrResult {
    fn merge(&mut self, mut other: EarcutrResult) {
        let base_triangle_index = self.vertices.len() / 2;
        for other_triangle_index in other.triangle_indices {
            self.triangle_indices
                .push(other_triangle_index + base_triangle_index);
        }
        self.vertices.append(&mut other.vertices);
    }
}

pub struct PolygonMeshBuilder {
    earcutr_inputs: Vec<EarcutrInput>,
    z_index: f32,
}

impl PolygonMeshBuilder {
    pub fn new() -> Self {
        PolygonMeshBuilder {
            earcutr_inputs: vec![],
            z_index: 0.,
        }
    }

    /*
    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }
    */

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_earcutr_input(&mut self, earcutr_input: EarcutrInput) {
        self.earcutr_inputs.push(earcutr_input);
    }

    pub fn build(self) -> Option<Mesh> {
        let z_index = self.z_index;
        let result = self.run_earcutr()?;
        Some(build_mesh_from_earcutr(result, z_index))
    }

    fn run_earcutr(self) -> Option<EarcutrResult> {
        let mut earcutr_inputs_iter = self.earcutr_inputs.into_iter();

        // Earcut the first polygon
        let first_input = earcutr_inputs_iter.next()?;
        let first_triangle_indices =
            earcutr::earcut(&first_input.vertices, &first_input.interior_indices, 2).unwrap();
        let mut earcutr_result = EarcutrResult {
            triangle_indices: first_triangle_indices,
            vertices: first_input.vertices,
        };

        // Earcut any additional polygons and merge the results into the result of the first polygon
        for earcutr_input in earcutr_inputs_iter {
            let EarcutrInput {
                vertices,
                interior_indices,
            } = earcutr_input;
            let next_earcutr_result = earcutr::earcut(&vertices, &interior_indices, 2).unwrap();
            earcutr_result.merge(EarcutrResult {
                triangle_indices: next_earcutr_result,
                vertices,
            });
        }

        Some(earcutr_result)
    }
}

pub fn build_mesh_from_earcutr(earcutr_result: EarcutrResult, z_index: f32) -> Mesh {
    let indices = earcutr_result
        .triangle_indices
        .into_iter()
        .map(|n| u32::try_from(n).unwrap())
        .collect::<Vec<_>>();
    let vertices = earcutr_result
        .vertices
        .chunks(2)
        .map(|n| [n[0] as f32, n[1] as f32, z_index])
        .collect::<Vec<_>>();
    build_mesh_from_bevy(indices, vertices)
}

fn build_mesh_from_bevy(triangle_indices: BevyIndices, vertices: BevyVertices) -> Mesh {
    let num_vertices = vertices.len();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_indices(Indices::U32(triangle_indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let normals = vec![[0.0, 0.0, 0.0]; num_vertices];
    let uvs = vec![[0.0, 0.0]; num_vertices];

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

pub fn generate_mesh_earcutr(path_2d: Vec<Vec2>, extrude_amount: f32) -> Mesh {
    // detect counter clockwise
    let mut path_2d = path_2d;
    let sum = path_2d
        .iter()
        .fold((path_2d.last().unwrap(), 0.0), |acc, item| {
            let last = acc.0;
            let result = (last.x - item.x) * (last.y + item.y);
            (item, acc.1 + result)
        });

    if sum.1 > 0.0 {
        path_2d.reverse();
    }

    let down = Vec3::NEG_Y;
    let up = Vec3::new(0.0, 1.0, 0.0);

    let y1 = 0.;
    let y2 = extrude_amount;

    let mut builder = MeshBuilder::new();
    let polygon = Polygon::new(
        LineString::new(
            path_2d
                .iter()
                .map(|p| coord! {x: p.x as f64, y: p.y as f64})
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    // Floor
    builder.triangulate_polygon(&polygon, y1, down);

    // Ceiling
    builder.triangulate_polygon(&polygon, y2, up);

    // For every line along the polygon, add a rectangular wall
    for line in polygon.exterior().lines() {
        let corner1 = Vec3::new(line.start.x as f32, y1, line.start.y as f32);
        let corner2 = Vec3::new(line.end.x as f32, y1, line.end.y as f32);
        let corner3 = Vec3::new(line.end.x as f32, y2, line.end.y as f32);
        let corner4 = Vec3::new(line.start.x as f32, y2, line.start.y as f32);

        // Now let's go fetch our buddy Norm
        let bottom_line = corner2 - corner1;
        let up_line = corner3 - corner2;
        let normal = bottom_line.cross(up_line).normalize();

        builder.add_quad([corner1, corner2, corner3, corner4], normal);
    }

    builder.build()
}
