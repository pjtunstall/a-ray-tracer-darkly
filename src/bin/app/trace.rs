#[macro_export]
macro_rules! trace {
    ($module:ident, $func:ident) => {{
        let image = rt::image::Image::new(400, 16.0 / 9.0);
        let viewport = rt::viewport::Viewport::new(2.0, &image);
        rt::render::render(
            &image,
            &viewport,
            concat!("example", stringify!($module)),
            $crate::examples::$module::$func,
        )
    }};
}
