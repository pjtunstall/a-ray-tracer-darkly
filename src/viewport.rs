use crate::vec3::direction::Direction;

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        let u = Direction::new(width, 0.0, 0.0);
        let v = Direction::new(0.0, -height, 0.0);
        Self {
            width,
            height,
            u,
            v,
        }
    }
}
