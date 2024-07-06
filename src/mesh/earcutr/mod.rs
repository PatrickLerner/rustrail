//! This code in file is generally borrowed from [bevy-earcutr](https://github.com/frewsxcv/bevy-earcutr)
//! With adjustments to make it work for our implementation. Nonetheless, you can consider it
//! Copyright (c) 2022 Corey Farwell, licensed under MIT as well

#[cfg(test)]
mod tests;

use super::mesh_builder::MeshBuilder;
use crate::landscape::CoordinatePoint;
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

fn build_mesh_from_earcutr(earcutr_result: EarcutrResult, z_index: f32) -> Mesh {
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

pub fn generate_2d_mesh(earcutr_input: EarcutrInput) -> Mesh {
    let triangle_indices =
        earcutr::earcut(&earcutr_input.vertices, &earcutr_input.interior_indices, 2).unwrap();

    let result = EarcutrResult {
        triangle_indices,
        vertices: earcutr_input.vertices,
    };

    build_mesh_from_earcutr(result, 0.0)
}

pub fn generate_3d_mesh(path_2d: Vec<CoordinatePoint>, extrude_amount: f32) -> Mesh {
    // detect if we are counter clockwise so that we can reverse the path if so
    let mut path_2d = path_2d;
    let sum = path_2d
        .iter()
        .fold((path_2d.last().unwrap(), 0.0), |acc, item| {
            let last = acc.0;
            let result = (last.0 - item.0) * (last.1 + item.1);
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
                .map(|p| coord! {x: p.0 , y: p.1})
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
