use crate::{
    image::Image,
    vec3::{self, Direction},
};

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Viewport {
    pub fn new(height: f64, image: &Image) -> Self {
        let width = height * (image.width as f64 / image.height as f64);
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
