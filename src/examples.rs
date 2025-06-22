pub mod cubes;
pub mod dull_metal;
pub mod glass;
pub mod lambertian;
pub mod random_spheres;
pub mod shiny_metal;
pub mod sky;

use std::io;

use crate::{
    camera::{Camera, CameraParameters},
    examples,
    vec3::{Direction, Point3},
};

pub fn book_1(max_depth: usize, samples_per_pixel: usize) -> io::Result<()> {
    let mut world;
    let background = examples::sky::color;
    let mut camera = set_up_camera();

    world = examples::lambertian::make();
    camera.render(
        &world,
        "example_1",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    world = examples::shiny_metal::make();
    camera.render(
        &world,
        "example_2",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    world = examples::dull_metal::make();
    camera.render(
        &world,
        "example_3",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    world = examples::glass::make();
    camera.render(
        &world,
        "example_4",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    camera = wide_angle();
    camera.render(
        &world,
        "example_5",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    camera = reduce_fov();
    camera.render(
        &world,
        "example_6",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    camera = defocus();
    camera.render(
        &world,
        "example_7",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    Ok(())
}

fn set_up_camera() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        look_from: Point3::new(0., 0., 4.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

pub fn wide_angle() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 90.,
    };
    Camera::new(params)
}

fn reduce_fov() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

fn defocus() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 3.4,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}
