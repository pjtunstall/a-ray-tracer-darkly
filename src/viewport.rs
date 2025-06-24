use crate::{image::Image, vec3::Direction};

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Viewport {
    pub fn new(height: f64, image: &Image, camera_u: &Direction, camera_v: &Direction) -> Self {
        assert!(
            height > f64::EPSILON,
            "Height of viewport must be greater than zero"
        );
        assert!(
            !camera_u.is_zero(),
            "camera_u direction vector must not be zero"
        );
        assert!(
            !camera_v.is_zero(),
            "camera_v direction vector must not be zero"
        );

        let width = height * (image.width as f64 / image.height as f64);
        let u = width * *camera_u;
        let v = -height * *camera_v;
        Self {
            width,
            height,
            u,
            v,
        }
    }
}
