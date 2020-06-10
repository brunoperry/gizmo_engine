extern crate js_sys;
use js_sys::Math;

extern crate wasm_bindgen;
use crate::renderer::*;
use wasm_bindgen::prelude::*;

use crate::vertex;
use vertex::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// extern crate renderer;
// // mod renderer;
// use renderer::*;

pub struct Stars3D {
    spread: f64,
    speed: f64,
    stars_x: Vec<f64>,
    stars_y: Vec<f64>,
    stars_z: Vec<f64>,
}

impl Stars3D {
    pub fn new(num_stars: usize, spread: f64, speed: f64) -> Self {
        let mut s_x = vec![0.; num_stars];
        let mut s_y = vec![0.; num_stars];
        let mut s_z = vec![0.; num_stars];

        for i in 0..num_stars {
            s_x[i] = 2. * (Math::random() - 0.5) * spread;
            s_y[i] = 2. * (Math::random() - 0.5) * spread;
            s_z[i] = (Math::random() + 0.00001) * spread;
        }

        Self {
            spread: spread,
            speed: speed,
            stars_x: s_x,
            stars_y: s_y,
            stars_z: s_z,
        }
    }

    pub fn init_star(&mut self, i: usize) {
        self.stars_x[i] = 2. * (Math::random() - 0.5) * self.spread;
        self.stars_y[i] = 2. * (Math::random() - 0.5) * self.spread;
        self.stars_z[i] = (Math::random() + 0.00001) * self.spread;
    }

    pub fn update_and_render(&mut self, target: &mut Renderer, delta: f64) {
        let tan_half_fov = Math::tan((90.0 / 2.0) * (3.14 / 180.));
        target.clear(0x00);

        let half_width = target.width as f64 / 2.0;
        let half_height = target.height as f64 / 2.0;
        let mut triangle_builder_counter = 0;

        let mut x1 = 0;
        let mut y1 = 0;
        let mut x2 = 0;
        let mut y2 = 0;

        for i in 0..self.stars_x.len() {
            self.stars_z[i] -= delta * self.speed;

            if self.stars_z[i] <= 0. {
                self.init_star(i);
            }

            let x = ((self.stars_x[i] / (self.stars_z[i] * tan_half_fov)) * half_width + half_width)
                as u32;
            let y = ((self.stars_y[i] / (self.stars_z[i] * tan_half_fov)) * half_height
                + half_height) as u32;

            if x <= 0 || x >= target.width || (y <= 0 || y >= target.height) {
                self.init_star(i);
                continue;
            }
            // else {
            //     target.draw(x, y, 0xFF, 0xFF, 0xFF, 0xFF);
            // }

            triangle_builder_counter += 1;
            if triangle_builder_counter == 1 {
                x1 = x;
                y1 = y;
            } else if triangle_builder_counter == 2 {
                x2 = x;
                y2 = y;
            } else if triangle_builder_counter == 3 {
                triangle_builder_counter = 0;
                let mut v1: Vertex = Vertex::new(x1 as f64, y1 as f64);
                let mut v2: Vertex = Vertex::new(x2 as f64, y2 as f64);
                let mut v3: Vertex = Vertex::new(x as f64, y as f64);

                target.fill_triangle(&mut v1, &mut v2, &mut v3);
            }
        }

        // target.render();
    }
}
