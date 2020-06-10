extern crate serde_json;
use serde::{Deserialize, Serialize};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[derive(Debug, Serialize, Deserialize)]
struct ResourcesData {
    objects_data: Vec<String>,
}

extern crate nalgebra as na;
use na::{Vector2, Vector4};

#[path = "engine/object_3d.rs"]
mod object_3d;
use object_3d::*;

pub struct Resources {
    pub objects_3d: Vec<Object3D>,
}

impl Resources {
    pub fn new(data: JsValue) -> Self {
        let resources_data: ResourcesData = data.into_serde().unwrap();
        Self {
            objects_3d: parse_objects(resources_data.objects_data),
        }
    }
}
fn parse_objects(data: Vec<String>) -> Vec<Object3D> {
    let mut objs_3d: Vec<Object3D> = Vec::new();
    for i in 0..=data.len() - 1 {
        let obj_data: Vec<&str> = data[i].split("\n").collect();
        let mut obj_vecs: Vec<Vector4<f32>> = Vec::new();
        let mut obj_uvs: Vec<Vector2<f32>> = Vec::new();
        let mut obj_normals: Vec<Vector2<f32>> = Vec::new();
        let mut obj_name: &str = "";
        for j in 0..=&obj_data.len() - 1 {
            let line: Vec<&str> = obj_data[j].split(" ").collect();

            if line[0] == "v" {
                obj_vecs.push(Vector4::new(
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                    line[3].parse::<f32>().unwrap(),
                    0.,
                ));
            } else if line[0] == "vt" {
                obj_uvs.push(Vector2::new(
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                ));
            } else if line[0] == "vn" {
                obj_normals.push(Vector2::new(
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                ))
            } else if line[0] == "g" {
                obj_name = line[1];
            }
        }

        objs_3d.push(Object3D::new(
            obj_vecs,
            obj_uvs,
            obj_normals,
            obj_name.to_string(),
        ));
    }

    objs_3d
}
