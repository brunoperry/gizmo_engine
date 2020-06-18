extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;

use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

use crate::vertex;
use vertex::*;

use crate::math;
use math::*;

mod edge;
use edge::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
    data: Vec<u8>,
    data_size: usize,
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn new() -> Self {
        let canvas_elem = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let context_2d = canvas_elem
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let size: usize = (canvas_elem.width() * canvas_elem.height() * 4) as usize;

        Self {
            context: context_2d,
            data: vec![0x00; size],
            data_size: size,
            width: canvas_elem.width(),
            height: canvas_elem.height(),
        }
    }

    pub fn render(&mut self) {
        log("render");

        self.context
            .put_image_data(
                &ImageData::new_with_u8_clamped_array_and_sh(
                    Clamped(self.data.as_mut()),
                    self.width,
                    self.height,
                )
                .unwrap(),
                0.0,
                0.0,
            )
            .unwrap();
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let index: usize = ((x + y * self.width) * 4) as usize;

        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
        self.data[index + 3] = a;
    }

    pub fn fill_triangle(&mut self, v1: &mut Vertex, v2: &mut Vertex, v3: &mut Vertex) {
        let screen_space_transform: Matrix4 =
            Matrix4::init_screen_space_transform(self.width as f32 / 2., self.height as f32 / 2.);

        let mut min_y_vert: Vertex = v1
            .transform(screen_space_transform.clone())
            .perspective_divide();
        let mut mid_y_vert: Vertex = v2
            .transform(screen_space_transform.clone())
            .perspective_divide();
        let mut max_y_vert: Vertex = v3
            .transform(screen_space_transform.clone())
            .perspective_divide();

        if max_y_vert.get_y() < mid_y_vert.get_y() {
            let temp: Vertex = max_y_vert;
            max_y_vert = mid_y_vert;
            mid_y_vert = temp;
        }

        if mid_y_vert.get_y() < min_y_vert.get_y() {
            let temp: Vertex = mid_y_vert;
            mid_y_vert = min_y_vert;
            min_y_vert = temp;
        }
        if max_y_vert.get_y() < mid_y_vert.get_y() {
            let temp: Vertex = max_y_vert;
            max_y_vert = mid_y_vert;
            mid_y_vert = temp;
        }

        self.scan_triangle(
            min_y_vert,
            mid_y_vert,
            max_y_vert,
            min_y_vert.triangle_area_times_two(max_y_vert, mid_y_vert) >= 0.,
        );
    }

    fn scan_triangle(
        &mut self,
        min_y_vert: Vertex,
        mid_y_vert: Vertex,
        max_y_vert: Vertex,
        handedness: bool,
    ) {
        let mut top_to_bottom: Edge = Edge::new(min_y_vert, max_y_vert);
        let mut top_to_middle: Edge = Edge::new(min_y_vert, mid_y_vert);
        let mut middle_to_bottom: Edge = Edge::new(mid_y_vert, max_y_vert);

        self.scan_edges(&mut top_to_bottom, &mut top_to_middle, handedness);

        self.scan_edges(&mut top_to_bottom, &mut middle_to_bottom, handedness);
    }

    fn scan_edges(&mut self, a: &mut Edge, b: &mut Edge, handedness: bool) {
        let mut left = a;
        let mut right = b;

        let y_start = right.get_y_start();
        let y_end = right.get_y_end();

        if handedness {
            let temp = left;
            left = right;
            right = temp;
        }

        for j in y_start..y_end {
            self.draw_scan_line(left, right, j);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&mut self, left: &mut Edge, right: &mut Edge, j: i32) {
        let x_min = left.get_x().ceil() as i32;
        let x_max = right.get_x().ceil() as i32;

        for i in x_min..x_max {
            self.draw_pixel(i as u32, j as u32, 0xFF, 0xFF, 0xFF, 0xFF);
        }
    }

    pub fn clear(&mut self, color: u8) {
        self.data = vec![color; self.data_size];
    }
}
