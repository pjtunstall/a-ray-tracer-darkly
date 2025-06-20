pub mod dull_metal;
pub mod glass;
pub mod lambertian;
pub mod random_spheres;
pub mod shiny_metal;
pub mod sky;

use std::{f64::consts::PI, io};

use crate::{
    camera::Camera,
    examples,
    vec3::{Direction, Point3},
};

pub fn book_1() -> io::Result<()> {
    let mut world;
    let background = examples::sky::color;
    let mut camera = set_up_camera();

    world = examples::lambertian::make();
    camera.render(&world, "example_1", 10, background)?; // antialiasing, fixing shadow acne, Lambertian reflection, gamma correction

    world = examples::shiny_metal::make();
    camera.render(&world, "example_2", 10, background)?;

    world = examples::dull_metal::make();
    camera.render(&world, "example_3", 10, background)?;

    world = examples::glass::make();
    camera.render(&world, "example_4", 10, background)?;

    camera = zoom_out();
    camera.render(&world, "example_5", 10, background)?;

    camera = reduce_fov();
    camera.render(&world, "example_6", 10, background)?;

    camera = defocus();
    camera.render(&world, "example_7", 10, background)?;

    random_spheres::render(500)?;

    Ok(())
}

fn set_up_camera() -> Camera {
    Camera::new(
        16.0 / 9.0,
        400,
        PI / 2.,
        Point3::new(0., 0., 0.),
        Point3::new(0., 0., -1.),
        Direction::new(0., 1., 0.),
        10.,
        0.,
        10,
    )
}

fn zoom_out() -> Camera {
    Camera::new(
        16.0 / 9.0,
        400,
        PI / 2.,
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Direction::new(0., 1., 0.),
        10.,
        0.,
        10,
    )
}

fn reduce_fov() -> Camera {
    Camera::new(
        16.0 / 9.0,
        400,
        PI / 9.,
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Direction::new(0., 1., 0.),
        10.,
        0.,
        10,
    )
}

fn defocus() -> Camera {
    Camera::new(
        16.0 / 9.0,
        400,
        PI / 9.,
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Direction::new(0., 1., 0.),
        10.,
        (3.4 as f64).to_radians(),
        10,
    )
}
