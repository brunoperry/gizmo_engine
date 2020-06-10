use crate::vertex;
use vertex::*;

pub struct Triangle {
    pub min_y_vert: Vertex,
    pub mid_y_vert: Vertex,
    pub max_y_vert: Vertex,
}

impl Triangle {
    pub fn new(min: Vertex, mid: Vertex, max: Vertex) -> Self {
        Self {
            min_y_vert: min,
            mid_y_vert: mid,
            max_y_vert: max,
        }
    }
}
