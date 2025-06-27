use crate::{image::Image, vec3::Direction};

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Viewport {
    pub fn new(height: f64, image: &Image, camera_u: &Direction, camera_v: &Direction) -> Self {
        assert!(height > 1e-8, "Height of viewport is too small");
        assert!(
            !camera_u.near_zero(),
            "camera_u direction vector too close to zero"
        );
        assert!(
            !camera_v.near_zero(),
            "camera_v direction vector too close to zero"
        );

        let width = height * (image.width as f64 / image.height as f64);
        let u = width * *camera_u;
        let v = -height * *camera_v; // Note the reversal of direction from `camera_v`! I've kept this to be consistent with Ray Tracing in One Weekend.

        Self {
            width,
            height,
            u,
            v,
        }
    }
}
