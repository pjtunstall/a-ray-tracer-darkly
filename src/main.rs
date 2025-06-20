use std::{f64::consts::PI, io};

use rt::{
    camera::Camera,
    examples,
    vec3::{Direction, Point3},
};

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let focus_dist = 10.;
    let defocus_angle = 0.;

    let image_width = 400;
    let mut vertical_fov = PI / 2.;
    let mut look_from = Point3::new(0., 0., 0.);
    let mut look_at = Point3::new(0., 0., -1.);
    let mut up = Direction::new(0., 1., 0.);
    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        vertical_fov,
        look_from,
        look_at,
        up,
        focus_dist,
        defocus_angle,
    );
    let mut world;

    world = examples::lambertian::make();
    camera.render(&world, "example_1", 10)?; // antialiasing, fixing shadow acne, Lambertian reflection, gamma correction

    world = examples::shiny_metal::make();
    camera.render(&world, "example_2", 10)?;

    world = examples::dull_metal::make();
    camera.render(&world, "example_3", 10)?;

    world = examples::glass::make();
    camera.render(&world, "example_4", 10)?;

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
        focus_dist,
        defocus_angle,
    );
    world = examples::glass::make();
    camera.render(&world, "example_5", 10)?;

    vertical_fov = PI / 9.;
    camera = Camera::new(
        aspect_ratio,
        image_width,
        vertical_fov,
        look_from,
        look_at,
        up,
        focus_dist,
        defocus_angle,
    );
    world = examples::glass::make();
    camera.render(&world, "example_6", 10)?;

    camera = Camera::new(
        aspect_ratio,
        image_width,
        vertical_fov,
        look_from,
        look_at,
        up,
        10.,
        (3.4 as f64).to_radians(),
    );
    world = examples::glass::make();
    camera.render(&world, "example_7", 50)?;

    Ok(())
}
