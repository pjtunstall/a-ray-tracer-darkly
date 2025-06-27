#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        assert!(1e-8 < aspect_ratio, "Aspect ratio is too low");

        let height = ((width as f64 / aspect_ratio) as u32).max(1);

        Self { width, height }
    }
}
