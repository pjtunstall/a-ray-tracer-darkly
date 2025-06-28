use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    examples,
    hittables::{HittableList, plane::Plane, sphere::Sphere},
    materials::{Dielectric, Lambertian, Metal},
    vec3::{Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make_world();
    let background = examples::book::sky::color;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("audit").join("sphere_scene"),
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
        image_width,
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
    let ground_material = Arc::new(Lambertian::new(Color::new(0.4, 0.6, 0.)));
    let center_material = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let left_material = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let right_material = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let rightmost_material = Arc::new(Dielectric::new(1.5));

    let ground = Arc::new(Plane::new(
        Point3::new(0., -0.5, 0.),
        Direction::new(0., 1., 0.),
        ground_material,
    ));
    let center = Arc::new(Sphere::new(Point3::new(0., 0., -2.5), 0.5, center_material));
    let left = Arc::new(Sphere::new(Point3::new(-0.5, 0., -3.), 0.5, left_material));
    let right = Arc::new(Sphere::new(Point3::new(1., 0., -1.5), 0.5, right_material));
    let rightmost = Arc::new(Sphere::new(
        Point3::new(1.3, 0., -0.5),
        0.5,
        rightmost_material,
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    world.add(rightmost);

    world
}
