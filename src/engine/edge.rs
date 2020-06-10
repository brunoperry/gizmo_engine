use crate::vertex;
use vertex::*;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    x: f64,
    x_step: f64,
    y_start: i32,
    y_end: i32,
}

impl Edge {
    pub fn new(min_y_vert: Vertex, max_y_vert: Vertex) -> Self {
        let ys = min_y_vert.get_y().ceil() as i32;
        let ye = max_y_vert.get_y().ceil() as i32;

        let y_dist: f64 = max_y_vert.get_y() - min_y_vert.get_y();
        let x_dist: f64 = max_y_vert.get_x() - min_y_vert.get_x();

        let xs: f64 = x_dist / y_dist;
        let yp: f64 = ys as f64 - min_y_vert.get_y();
        let xx: f64 = min_y_vert.get_x() + yp * xs;

        Self {
            x: xx,
            x_step: xs,
            y_start: ys,
            y_end: ye,
        }
    }

    pub fn step(&mut self) {
        self.x += self.x_step;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y_start(&self) -> i32 {
        self.y_start
    }
    pub fn get_y_end(&self) -> i32 {
        self.y_end
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let data: String = [
            "x: ",
            &self.x.to_string()[..],
            "\nx_step: ",
            &self.x_step.to_string()[..],
            "\ny_start: ",
            &self.y_start.to_string()[..],
            "\ny_end: ",
            &self.y_end.to_string()[..],
        ]
        .concat();

        data
    }
}
