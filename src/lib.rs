extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[path = "engine/renderer.rs"]
mod renderer;
use renderer::*;

#[path = "engine/vertex.rs"]
mod vertex;
use vertex::*;

#[path = "engine/triangle.rs"]
mod triangle;
use triangle::*;

#[path = "engine/math.rs"]
mod math;
use math::*;

mod resources;
use resources::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    renderer: Renderer,
    rot_counter: f64,
    triangle: Triangle,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(resources_data: JsValue) -> Self {
        Self {
            renderer: Renderer::new(),
            rot_counter: 2.5,
            triangle: Triangle::new(
                Vertex::new(Vector4::new(-1., -1., 0., 1.)),
                Vertex::new(Vector4::new(0., 1., 0., 1.)),
                Vertex::new(Vector4::new(1., -1., 0., 1.)),
            ),
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.renderer.clear(0x00);

        let projection = Matrix4::init_perspective(
            (70.0f64).to_radians(),
            self.renderer.width as f64 / self.renderer.height as f64,
            0.1,
            1000.,
        );

        self.rot_counter += 0.03;

        // log(&self.rot_counter.to_string()[..]);

        let translation: Matrix4 = Matrix4::init_translation(0., 0., 3.);
        let rotation: Matrix4 = Matrix4::init_rotation(0., self.rot_counter, 0.);
        let transform: Matrix4 = projection * (translation * rotation);

        self.renderer.fill_triangle(
            &mut self.triangle.max_y_vert.transform(transform.clone()),
            &mut self.triangle.mid_y_vert.transform(transform.clone()),
            &mut self.triangle.min_y_vert.transform(transform.clone()),
        );

        // self.renderer.fill_triangle(
        //     &mut self.triangle.max_y_vert.transform(transform.clone()),
        //     &mut self.triangle.mid_y_vert.transform(transform.clone()),
        //     &mut self.triangle.min_y_vert.transform(transform.clone()),
        // );

        self.renderer.render();
    }

    #[wasm_bindgen]
    pub fn draw(&mut self) {
        self.update();
    }

    #[wasm_bindgen]
    pub fn clear(&mut self, color: u8) {
        self.renderer.clear(color);
        self.renderer.render();
    }
}
