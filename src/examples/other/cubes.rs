use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    examples,
    hittables::{HittableList, cube::Cube, sphere::Sphere},
    material::{Dielectric, Metal},
    vec3::{Basis, Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize) -> io::Result<()> {
    let world = make();

    let background = examples::book::sky::color;
    let camera = set_up_camera();
    camera.render(
        &world,
        PathBuf::from("other").join("cubes"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    Ok(())
}

fn set_up_camera() -> Camera {
    let params = CameraParameters {
        aspect_ratio: 4.0 / 3.0,
        image_width: 400,
        look_from: Point3::new(0., 6., 9.),
        look_at: Point3::new(0., 0., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 30.,
    };
    Camera::new(params)
}

fn make() -> HittableList {
    let earth = Arc::new(Dielectric::new(1.33));
    let metal_1 = Arc::new(Metal::new(Color::new(0.1, 0.2, 0.5), 0.5));
    let glass = Arc::new(Dielectric::new(1.5));
    let metal_2 = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    let ground = Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100., earth));

    let center = Box::new(Cube::new_oriented(
        Point3::new(0.0, 0., -1.),
        0.3,
        &Basis::new_orthonormal(),
        glass.clone(),
    ));
    let inner = Box::new(Cube::new_oriented(
        Point3::new(0.0, 0., -1.),
        0.2,
        &Basis::new_orthonormal(),
        glass.clone(),
    ));
    let inmost = Box::new(Cube::new_oriented(
        Point3::new(0.0, 0., -1.),
        0.1,
        &Basis::new_orthonormal(),
        metal_2.clone(),
    ));

    let orientation = Basis::new_orthonormal();
    let left = Box::new(Cube::new_oriented(
        Point3::new(-1., 0., -1.),
        0.2,
        &orientation,
        metal_1.clone(),
    ));

    let right = Box::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, metal_2.clone()));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    world.add(inner);
    world.add(inmost);

    world
}
