mod examples;

use std::{f64::consts::PI, io};

use rt::{
    camera::Camera,
    vec3::{Direction, Point3},
};

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut vertical_fov = PI / 2.;
    let mut look_from = Point3::new(0., 0., 0.);
    let mut up = Direction::new(0., 1., 0.);
    let mut look_at = Point3::new(0., 0., -1.);
    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        vertical_fov,
        look_from,
        look_at,
        up,
    );
    let mut world;

    world = examples::lambertian::gamma();
    camera.render(&world, "example_5", 10)?; // antialiasing, fixing shadow acne, Lambertian reflection, gamma correction

    world = examples::shiny_metal::shiny_metal();
    camera.render(&world, "example_6", 10)?;

    world = examples::dull_metal::fuzzy_metal();
    camera.render(&world, "example_7", 10)?;

    world = examples::glass::hollow_glass();
    camera.render(&world, "example_8", 10)?;

    look_from = Point3::new(-2., 2., 1.);
    up = Direction::new(0., 1., 0.);
    look_at = Point3::new(0., 0., -1.);
    camera = Camera::new(
        aspect_ratio,
        image_width,
        vertical_fov,
        look_from,
        look_at,
        up,
    );
    world = examples::glass::hollow_glass();
    camera.render(&world, "example_9", 10)?;

    vertical_fov = PI / 9.;
    camera = Camera::new(
        aspect_ratio,
        image_width,
        vertical_fov,
        look_from,
        look_at,
        up,
    );
    world = examples::glass::hollow_glass();
    camera.render(&world, "example_10", 10)?;

    Ok(())
}
