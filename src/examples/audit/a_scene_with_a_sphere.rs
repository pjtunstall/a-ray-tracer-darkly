use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    examples,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    plane::Plane,
    sphere::Sphere,
    vec3::{Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
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
        aspect_ratio: 4.0 / 3.0,
        image_width: image_width,
        look_from: Point3::new(0., 1., 4.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

pub fn make_world() -> HittableList {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.4, 0.6, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let material_rightmost = Arc::new(Dielectric::new(1.5));

    let ground = Box::new(Plane::from_span(
        Point3::new(0., -0.5, 0.),
        Direction::new(1., 0., 0.),
        Direction::new(0., 0., 1.),
        material_ground,
    ));
    let center = Box::new(Sphere::new(
        Point3::new(0., 0., -2.5),
        0.5,
        material_center.clone(),
    ));
    let left = Box::new(Sphere::new(
        Point3::new(-0.5, 0., -3.),
        0.5,
        material_left.clone(),
    ));
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.5),
        0.5,
        material_right.clone(),
    ));
    let rightmost = Box::new(Sphere::new(
        Point3::new(1.3, 0., -0.5),
        0.5,
        material_rightmost,
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    world.add(rightmost);

    world
}
