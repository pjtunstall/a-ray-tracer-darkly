use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    cube::Cube,
    examples,
    hittable::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::{Basis, Direction, Point3},
};

pub fn cube_and_plane(
    max_depth: usize,
    samples_per_pixel: usize,
    image_width: u32,
) -> io::Result<()> {
    let world = make_world();
    let background = examples::book::sky::color;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("audit").join("plane_and_cube"),
        max_depth,
        samples_per_pixel,
        background,
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
    let earth = Arc::new(Lambertian::new(Color::new(0.5, 0.3, 0.0)));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.1, 0.5), 0.2));

    let ground = Box::new(Sphere::new(Point3::new(0., -666.5, -1.), 666., earth));

    let center = Box::new(Cube::new_oriented(
        Point3::new(0.0, 0., -1.),
        0.3,
        metal.clone(),
        &Basis::new_orthonormal(),
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);

    world
}
