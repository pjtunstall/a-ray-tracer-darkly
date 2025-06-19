mod examples;
mod trace;

use std::io;

use rt::camera::Camera;

fn main() -> io::Result<()> {
    examples::_1::gradient()?;

    trace!(_2, sky)?;
    trace!(_3, red_sphere)?;
    trace!(_4, color_map_sphere)?;

    // Now we start using the new `render` method in `Camera`.
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut camera = Camera::new(aspect_ratio, image_width);
    let world = examples::_5::ground();
    camera.render(&world, "example_5")?; // antialiasing, Lambertian reflection, gamma correction

    Ok(())
}
