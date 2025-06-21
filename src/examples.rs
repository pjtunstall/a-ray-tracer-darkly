pub mod dull_metal;
pub mod glass;
pub mod lambertian;
pub mod random_spheres;
pub mod shiny_metal;
pub mod sky;

use std::{f64::consts::PI, io};

use crate::{
    camera::{Camera, CameraParameters},
    examples,
    vec3::{Direction, Point3},
};

pub fn book_1(samples_per_pixel_for_the_big_picture: usize) -> io::Result<()> {
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

    random_spheres::render(samples_per_pixel_for_the_big_picture)?;

    Ok(())
}

fn set_up_camera() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vertical_fov: PI / 2.,
        look_from: Point3::new(0., 0., 0.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focus_dist: 10.,
        defocus_angle: 0.,
        max_depth: 10,
    };
    Camera::new(params)
}

fn zoom_out() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vertical_fov: PI / 2.,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focus_dist: 10.,
        defocus_angle: 0.,
        max_depth: 10,
    };
    Camera::new(params)
}

fn reduce_fov() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vertical_fov: PI / 9.,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focus_dist: 10.,
        defocus_angle: 0.,
        max_depth: 10,
    };
    Camera::new(params)
}

fn defocus() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vertical_fov: PI / 9.,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focus_dist: 10.,
        defocus_angle: 3.4_f64.to_radians(),
        max_depth: 10,
    };
    Camera::new(params)
}
