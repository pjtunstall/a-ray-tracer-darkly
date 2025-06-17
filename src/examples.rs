pub mod _1;
pub mod _2;
pub mod _3;

use rt::{image::Image, viewport::Viewport};

#[macro_export]
macro_rules! trace {
    ($module:ident, $func:ident) => {{
        let (image, viewport) = $crate::examples::make_image_and_viewport(16.0 / 9.0, 2.0, 400);
        rt::render::render(
            &image,
            &viewport,
            concat!("example", stringify!($module)),
            examples::$module::$func,
        )
    }};
}

pub fn make_image_and_viewport(
    aspect_ratio: f64,
    viewport_height: f64,
    image_width: u32,
) -> (Image, Viewport) {
    let image = Image::new(image_width, aspect_ratio);
    let viewport_width = viewport_height * (image.width as f64 / image.height as f64);
    let viewport = Viewport::new(viewport_width, viewport_height);
    (image, viewport)
}
