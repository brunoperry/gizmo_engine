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
    scan_buffer: Vec<i32>,
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
            scan_buffer: vec![0; (canvas_elem.height() * 2) as usize],
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

    pub fn draw_scan_buffer(&mut self, y_coord: i32, x_min: i32, x_max: i32) {
        self.scan_buffer[(y_coord * 2) as usize] = x_min;
        self.scan_buffer[(y_coord * 2 + 1) as usize] = x_max;
    }

    pub fn fill_shape(&mut self, y_min: i32, y_max: i32) {
        for j in y_min..y_max {
            let x_min = self.scan_buffer[(j * 2) as usize];
            let x_max = self.scan_buffer[(j * 2 + 1) as usize];

            for i in x_min..x_max {
                self.draw_pixel(i as u32, j as u32, 0xFF, 0xFF, 0xFF, 0xFF);
            }
        }
    }

    pub fn fill_triangle(&mut self, v1: &mut Vertex, v2: &mut Vertex, v3: &mut Vertex) {
        let screen_space_transform: Matrix4 =
            Matrix4::init_screen_space_transform(self.width as f64 / 2., self.height as f64 / 2.);
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

        let area: f64 = min_y_vert.triangle_area_times_two(max_y_vert, mid_y_vert);

        let handedness = if area >= 0. { 1 } else { 0 };

        self.scan_convert_triangle(min_y_vert, mid_y_vert, max_y_vert, handedness);
        self.fill_shape(
            min_y_vert.get_y().ceil() as i32,
            max_y_vert.get_y().ceil() as i32,
        );
    }

    pub fn scan_convert_triangle(
        &mut self,
        min_y_vert: Vertex,
        mid_y_vert: Vertex,
        max_y_vert: Vertex,
        handedness: i32,
    ) {
        self.scan_convert_line(min_y_vert, max_y_vert, 0 + handedness);
        self.scan_convert_line(min_y_vert, mid_y_vert, 1 - handedness);
        self.scan_convert_line(mid_y_vert, max_y_vert, 1 - handedness);
    }

    fn scan_convert_line(&mut self, min_y_vert: Vertex, max_y_vert: Vertex, which_side: i32) {
        let y_start: i32 = min_y_vert.get_y().ceil() as i32;
        let y_end: i32 = max_y_vert.get_y().ceil() as i32;
        // let x_start: i32 = min_y_vert.get_x().ceil() as i32;
        // let x_end: i32 = max_y_vert.get_x().ceil() as i32;

        let y_dist: f64 = max_y_vert.get_y() - min_y_vert.get_y();
        let x_dist: f64 = max_y_vert.get_x() - min_y_vert.get_x();

        if y_dist <= 0. {
            return;
        }

        let x_step: f64 = x_dist / y_dist;
        let y_prestep: f64 = y_start as f64 - min_y_vert.get_y();
        let mut cur_x: f64 = min_y_vert.get_x() + y_prestep * x_step;

        for j in y_start..y_end {
            self.scan_buffer[(j * 2 + which_side) as usize] = cur_x.ceil() as i32;
            cur_x += x_step;
        }
    }

    // pub fn fill_triangle(&mut self, v1: &mut Vertex, v2: &mut Vertex, v3: &mut Vertex) {
    //     let screen_space_transform =
    //         Vertex::init_screen_space_transform(self.width as f64 / 2., self.height as f64 / 2.);

    //     let mut min_y_vert: Vertex = v1
    //         .transform(screen_space_transform.clone())
    //         .perspective_divide();
    //     let mut mid_y_vert: Vertex = v2
    //         .transform(screen_space_transform.clone())
    //         .perspective_divide();
    //     let mut max_y_vert: Vertex = v3
    //         .transform(screen_space_transform.clone())
    //         .perspective_divide();

    //     if max_y_vert.get_y() < mid_y_vert.get_y() {
    //         let tmp: Vertex = max_y_vert;
    //         max_y_vert = mid_y_vert;
    //         mid_y_vert = tmp;
    //     }
    //     if mid_y_vert.get_y() < min_y_vert.get_y() {
    //         let tmp: Vertex = mid_y_vert;
    //         mid_y_vert = min_y_vert;
    //         min_y_vert = tmp;
    //     }
    //     if max_y_vert.get_y() < mid_y_vert.get_y() {
    //         let tmp: Vertex = max_y_vert;
    //         max_y_vert = mid_y_vert;
    //         mid_y_vert = tmp;
    //     }

    //     self.scan_triangle(
    //         min_y_vert,
    //         mid_y_vert,
    //         max_y_vert,
    //         min_y_vert.triangle_area_times_two(max_y_vert, mid_y_vert) >= 0.,
    //     );
    // }

    // pub fn scan_triangle(
    //     &mut self,
    //     min_y_vert: Vertex,
    //     mid_y_vert: Vertex,
    //     max_y_vert: Vertex,
    //     handedness: bool,
    // ) {
    //     let top_to_bottom: Edge = Edge::new(min_y_vert, max_y_vert);
    //     let top_to_middle: Edge = Edge::new(min_y_vert, mid_y_vert);
    //     let middle_to_bottom: Edge = Edge::new(mid_y_vert, max_y_vert);

    //     self.scan_edges(top_to_bottom, top_to_middle, handedness);
    //     self.scan_edges(top_to_bottom, middle_to_bottom, handedness);
    // }

    // pub fn scan_edges(&mut self, a: Edge, b: Edge, handedness: bool) {
    //     let mut left = a;
    //     let mut right = b;

    //     if handedness {
    //         let temp = left;
    //         left = right;
    //         right = temp;
    //     }

    //     let y_start = b.get_y_start();
    //     let y_end = b.get_y_end();

    //     for j in y_start..y_end {
    //         self.draw_scanline(left, right, j as u32);
    //         left.step();
    //         right.step();
    //     }
    // }

    // pub fn draw_scanline(&mut self, left: Edge, right: Edge, j: u32) {
    //     let x_min = left.get_x().ceil() as u32;
    //     let x_max = right.get_x().ceil() as u32;

    //     for i in x_min..x_max {
    //         self.draw_pixel(i, j, 0xff, 0xff, 0xff, 0xff);
    //     }
    // }

    pub fn clear(&mut self, color: u8) {
        self.data = vec![color; self.data_size];
    }
}
