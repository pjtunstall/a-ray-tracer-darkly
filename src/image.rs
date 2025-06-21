#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        let height = ((width as f64 / aspect_ratio) as u32).max(1);

        Self { width, height }
    }
}
