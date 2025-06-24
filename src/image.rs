#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        assert!(width > 0, "Image width must be greater than zero");
        assert!(
            aspect_ratio > f64::EPSILON,
            "Aspect ratio must be greater than zero"
        );

        let height = ((width as f64 / aspect_ratio) as u32).max(1);

        Self { width, height }
    }
}
