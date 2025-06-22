use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    examples,
    hittable::HittableList,
    material::Lambertian,
    sphere::Sphere,
    vec3::{Direction, Point3},
};

pub fn a_scene_with_a_sphere(
    max_depth: usize,
    samples_per_pixel: usize,
    image_width: u32,
) -> io::Result<()> {
    let world = make_world();
    let background = examples::book::sky::color;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("audit").join("a_scene_with_a_sphere"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    Ok(())
}

fn set_up_camera(image_width: u32) -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: image_width,
        look_from: Point3::new(0., 0., 4.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

fn make_world() -> HittableList {
    let lambertian = Arc::new(Lambertian {
        albedo: Color::new(0.7, 0.1, 0.4),
    });

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0., -1.),
        0.5,
        lambertian.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        lambertian.clone(),
    )));

    world
}
