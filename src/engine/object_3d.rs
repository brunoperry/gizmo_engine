extern crate nalgebra as na;
use na::{Vector2, Vector4};
pub struct Object3D {
    vertices: Vec<Vector4<f32>>,
    uvs: Vec<Vector2<f32>>,
    normals: Vec<Vector2<f32>>,
    name: String,
}

impl Object3D {
    pub fn new(
        verts: Vec<Vector4<f32>>,
        uvs: Vec<Vector2<f32>>,
        normals: Vec<Vector2<f32>>,
        name: String,
    ) -> Self {
        Self {
            vertices: verts,
            uvs: uvs,
            normals: normals,
            name: name,
        }
    }
}
