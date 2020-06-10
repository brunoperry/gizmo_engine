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
        let ys = min_y_vert.get_y() as i32;
        let ye = max_y_vert.get_y() as i32;

        let y_dist = max_y_vert.get_y() - min_y_vert.get_y();
        let x_dist = max_y_vert.get_x() - min_y_vert.get_x();

        let y_pre_step = (ys as f64) - min_y_vert.get_y();

        let xs = x_dist / y_dist;
        let xx = min_y_vert.get_x() + y_pre_step * xs;

        Self {
            y_start: ys,
            y_end: ye,
            x_step: xs,
            x: xx,
        }
    }

    pub fn step(&mut self) {
        self.x += self.x_step;
    }

    pub fn get_x(self) -> f64 {
        self.x
    }
    pub fn get_y_start(self) -> i32 {
        self.y_start
    }

    pub fn get_y_end(self) -> i32 {
        self.y_end
    }
}
