use crate::vec3::{self, Direction};

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        let u = vec3::direction(width, 0.0, 0.0);
        let v = vec3::direction(0.0, -height, 0.0);
        Self {
            width,
            height,
            u,
            v,
        }
    }
}
