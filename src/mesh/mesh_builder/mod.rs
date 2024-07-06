#[cfg(test)]
mod tests;

use super::earcutr::{EarcutrInput, PolygonMeshBuilder};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};
use geo::{CoordsIter, LineString, Polygon};

pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

pub struct MeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    // Returns index
    pub fn add_vertex(&mut self, vert: Vertex) -> u32 {
        self.vertices.push(vert);
        (self.vertices.len() - 1) as u32
    }

    pub fn add_triangle(&mut self, i1: u32, i2: u32, i3: u32) {
        self.indices.extend([i1, i2, i3]);
    }

    pub fn add_quad(&mut self, positions: [Vec3; 4], normal: Vec3) {
        let c1 = self.add_vertex(Vertex {
            pos: positions[0],
            normal,
            uv: Vec2::new(0.0, 0.0),
        });
        let c2 = self.add_vertex(Vertex {
            pos: positions[1],
            normal,
            uv: Vec2::new(0.0, 1.0),
        });
        let c3 = self.add_vertex(Vertex {
            pos: positions[2],
            normal,
            uv: Vec2::new(1.0, 1.0),
        });
        let c4 = self.add_vertex(Vertex {
            pos: positions[3],
            normal,
            uv: Vec2::new(1.0, 0.0),
        });
        self.add_triangle(c1, c2, c3);
        self.add_triangle(c3, c4, c1);
    }

    // Adds a polygon in the XZ plane
    pub fn triangulate_polygon(&mut self, polygon: &Polygon, y: f32, normal: Vec3) {
        let mut builder = PolygonMeshBuilder::new();
        builder.add_earcutr_input(polygon_to_earcutr_input(polygon));
        let mesh = builder.build().unwrap();

        // Extract positions from the mesh. It'll use XY and ignore Z, but we use XZ
        let offset = self.vertices.len() as u32;
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            for pos in positions {
                self.add_vertex(Vertex {
                    pos: Vec3 {
                        x: pos[0],
                        y,
                        z: pos[1],
                    },
                    normal,
                    uv: Vec2::new(0.0, 0.0),
                });
            }
        } else {
            unreachable!()
        }
        if let Some(Indices::U32(indices)) = mesh.indices() {
            for idx in indices {
                self.indices.push(offset + idx);
            }
            // Somehow reversing the indices seems to fix the culling issues.
            // Most likely a winding order related.
            self.indices.reverse();
        } else {
            unreachable!()
        }
    }

    pub fn build(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        mesh.insert_indices(Indices::U32(self.indices));

        let mut position = Vec::new();
        let mut normal = Vec::new();
        let mut uv = Vec::new();
        for vert in self.vertices {
            position.push(vert.pos);
            normal.push(vert.normal);
            uv.push(vert.uv);
        }

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            position.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normal.iter().map(|n| n.to_array()).collect::<Vec<_>>(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uv.iter().map(|n| n.to_array()).collect::<Vec<_>>(),
        );

        mesh
    }
}

// Copied from rgis/geo-bevy
fn polygon_to_earcutr_input(polygon: &Polygon) -> EarcutrInput {
    let mut vertices = Vec::with_capacity(polygon.coords_count() * 2);
    let mut interior_indices = Vec::with_capacity(polygon.interiors().len());
    debug_assert!(polygon.exterior().0.len() >= 4);

    flat_line_string_coords_2(polygon.exterior(), &mut vertices);

    for interior in polygon.interiors() {
        debug_assert!(interior.0.len() >= 4);
        interior_indices.push(vertices.len() / 2);
        flat_line_string_coords_2(interior, &mut vertices);
    }

    EarcutrInput {
        vertices,
        interior_indices,
    }
}

fn flat_line_string_coords_2(line_string: &LineString, vertices: &mut Vec<f64>) {
    for coord in &line_string.0 {
        vertices.push(coord.x);
        vertices.push(coord.y);
    }
}
