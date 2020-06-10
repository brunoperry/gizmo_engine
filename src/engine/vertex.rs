use crate::math;
use math::*;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: Vector4,
}

impl Vertex {
    pub fn new(pos: Vector4) -> Self {
        Self { pos: pos }
    }

    pub fn init_screen_space_transform(half_width: f64, half_height: f64) -> Matrix4 {
        Matrix4::init_screen_space_transform(half_width, half_height)
    }

    pub fn perspective_divide(&self) -> Vertex {
        Vertex::new(Vector4::new(
            self.pos.x / self.pos.w,
            self.pos.y / self.pos.w,
            self.pos.z / self.pos.w,
            self.pos.w,
        ))
    }
    pub fn transform(&mut self, transform: Matrix4) -> Vertex {
        Vertex::new(transform.transform(self.pos))
    }

    pub fn triangle_area_times_two(&self, b: Vertex, c: Vertex) -> f64 {
        let x1 = b.get_x() - self.pos.x;
        let y1 = b.get_y() - self.pos.y;

        let x2 = c.get_x() - self.pos.x;
        let y2 = c.get_y() - self.pos.y;

        x1 * y2 - x2 * y1
    }

    pub fn get_x(self) -> f64 {
        self.pos.x
    }
    pub fn get_y(self) -> f64 {
        self.pos.y
    }
    pub fn get_z(self) -> f64 {
        self.pos.z
    }
    pub fn get_w(self) -> f64 {
        self.pos.w
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let data: String = [
            "x: ",
            &self.get_x().to_string()[..],
            "\ny: ",
            &self.get_y().to_string()[..],
            "\nz: ",
            &self.get_z().to_string()[..],
            "\nw: ",
            &self.get_w().to_string()[..],
        ]
        .concat();

        data
    }
}
