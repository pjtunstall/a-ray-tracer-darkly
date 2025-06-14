use crate::vec3::{Direction, DirectionType, Vec3};

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        let u = Vec3::<DirectionType>::new(width, 0.0, 0.0);
        let v = Vec3::<DirectionType>::new(0.0, -height, 0.0);
        Self {
            width,
            height,
            u,
            v,
        }
    }
}
