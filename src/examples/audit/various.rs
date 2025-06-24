use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    cube::Cube,
    disk::Disk,
    examples,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    quad::Quad,
    sphere::Sphere,
    tube::Tube,
    vec3::{Basis, Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make_world();
    let background = examples::book::sky::color;

    let mut look_from = Point3::new(0., 1., 6.);
    let mut camera = set_up_camera(image_width, look_from);
    camera.render(
        &world,
        PathBuf::from("audit").join("a_scene_with_various_objects"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    look_from = Point3::new(-9., 3., -12.);
    camera = set_up_camera(image_width, look_from);
    camera.render(
        &world,
        PathBuf::from("audit").join("the_same_scene_from_a_new_pov"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    Ok(())
}

fn set_up_camera(image_width: u32, look_from: Point3) -> Camera {
    let params = CameraParameters {
        aspect_ratio: 4.0 / 3.0,
        image_width: image_width,
        look_from,
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

fn make_world() -> HittableList {
    let earth = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.4),
    });
    let metal = Arc::new(Metal::new(Color::new(1., 1., 1.), 0.1));
    let glass = Arc::new(Dielectric::new(1.5));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -666.5, -1.),
        666.,
        earth.clone(),
    ));
    let cube = Box::new(Cube::new_oriented(
        Point3::new(-0.5, 0., -1.),
        0.3,
        metal.clone(),
        &Basis::new_orthonormal(),
    ));
    let sphere = Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.3, metal.clone()));
    let disk1 = Box::new(Disk::new(
        Point3::new(0., 0.3, -1.),
        0.8,
        Direction::new(1., 0., 0.),
        Direction::new(0., 0., 1.),
        glass.clone(),
    ));
    let disk2 = Box::new(Disk::new(
        Point3::new(0., 1.3, -1.),
        0.5,
        Direction::new(1., 0., 0.),
        Direction::new(0., 1., 0.),
        earth.clone(),
    ));
    let quad = Box::new(Quad::new(
        Point3::new(0.5, 0.2, -1.),
        Direction::new(1., 0., -1.),
        Direction::new(0., 1., 0.),
        earth.clone(),
    ));
    let tube = Box::new(Tube::new(
        Point3::new(0.4, 0., -1.),
        Direction::new(1.5, 1.5, -0.4),
        1.,
        earth,
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(sphere);
    world.add(disk1);
    world.add(disk2);
    world.add(cube);
    world.add(quad);
    world.add(tube);

    world
}
