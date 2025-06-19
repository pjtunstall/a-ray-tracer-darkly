mod examples;
mod trace;

use std::io;

use rt::camera::Camera;

fn main() -> io::Result<()> {
    examples::_1::gradient()?;

    trace!(_2, sky)?;
    trace!(_3, red_sphere)?;
    trace!(_4, color_map_sphere)?;

    // Now we start using the `render` method in `Camera`.
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut camera = Camera::new(aspect_ratio, image_width);
    let mut world;

    world = examples::_5::gamma();
    camera.render(&world, "example_5", 10)?; // antialiasing, fixing shadow acne, Lambertian reflection, gamma correction

    world = examples::_6::shiny_metal();
    camera.render(&world, "example_6", 10)?;

    world = examples::_7::fuzzy_metal();
    camera.render(&world, "example_7", 10)?;

    world = examples::_8::hollow_glass();
    camera.render(&world, "example_8", 10)?;

    Ok(())
}
