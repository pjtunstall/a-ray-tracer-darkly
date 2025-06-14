pub struct Image {
    pub width: u32,
    pub height: u32,
}

pub struct Viewport {
    pub width: f64,
    pub height: f64,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        let height = ((width as f64 / aspect_ratio) as u32).max(1);

        Self { width, height }
    }
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

pub fn make_image_and_viewport() -> (Image, Viewport) {
    let aspect_ratio = 16.0 / 9.0;
    let image = Image::new(400, aspect_ratio);
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image.width as f64 / image.height as f64);
    let viewport = Viewport::new(viewport_width, viewport_height);
    (image, viewport)
}
