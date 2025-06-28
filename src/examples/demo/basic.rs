use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    hittables::{HittableList, plane::Plane, sphere::Sphere},
    materials::Lambertian,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let camera = set_up_camera(image_width);
    let world = create_world();
    let background = sky;

    let brightness = 1.0;

    camera.render(
        &world,
        PathBuf::from("demo").join("basic"),
        max_depth,
        samples_per_pixel,
        background,
        brightness,
    )?;

    Ok(())
}

fn sky(_ray: &Ray) -> Color {
    Color::new(0.8, 0.8, 0.9)
}

fn set_up_camera(image_width: u32) -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width,
        look_from: Point3::new(0.0, 0.2, 4.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        up: Direction::new(0.0, 1.0, 0.0),
        focal_distance: 10.0,
        defocus_angle_in_degrees: 0.0,
        vertical_fov_in_degrees: 20.0,
    };

    Camera::new(params)
}

fn create_world() -> HittableList {
    let ground_color = Color::new(0.5, 0.5, 0.5);
    let ground_material = Arc::new(Lambertian::new(ground_color));
    let plane = Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Direction::new(0.0, 1.0, 0.0),
        ground_material,
    );
    let ground = Box::new(plane);

    let sphere_color = Color::new(0.8, 0.4, 0.4);
    let sphere_material = Arc::new(Lambertian::new(sphere_color));
    let center = Point3::new(0.0, 0.0, -2.5);
    let radius = 0.5;
    let sphere = Box::new(Sphere::new(center, radius, sphere_material.clone()));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(sphere);

    world
}
