use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::{self, Color},
    hittables::{HittableList, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
    materials::{Dielectric, Lambertian, Metal},
    ray::Ray,
    vec3::{Basis, Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make_world();
    let background = sky;
    let source_1 = Point3::new(-9., 7., -12.);
    let source_2 = Point3::new(-0.5, 1., 7.);

    let mut camera = set_up_camera(image_width, source_1);
    camera.render(
        &world,
        PathBuf::from("audit").join("various_pov_1"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    camera = set_up_camera(image_width, source_2);
    camera.render(
        &world,
        PathBuf::from("audit").join("various_pov_2"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    Ok(())
}

fn sky(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    let horizon = Color::new(0.8, 0.6, 0.4);
    let zenith = Color::new(0.2, 0.3, 0.5);
    color::lerp(horizon, zenith, t)
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
    let ground_color = Color::new(0.2, 0.4, 0.4);
    let rightmost_color = Color::new(0.9, 0.1, 0.2);
    let cube_color = Color::new(0., 0.7, 0.0);
    let material_cube = Arc::new(Lambertian::new(cube_color));
    let material_ground = Arc::new(Lambertian::new(ground_color));
    let material_rightmost = Arc::new(Lambertian::new(rightmost_color));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let glass = Arc::new(Dielectric::new(1.5));

    let ground = Arc::new(Plane::from_span(
        Point3::new(0., -0.5, 0.),
        Direction::new(1., 0., 0.),
        Direction::new(0., 0., 1.),
        material_ground.clone(),
    ));
    let left = Arc::new(Sphere::new(
        Point3::new(-0.5, 0.3, -3.),
        0.5,
        material_left.clone(),
    ));
    let right = Arc::new(Sphere::new(
        Point3::new(1., 0.5, -1.5),
        0.5,
        material_right.clone(),
    ));
    let rightmost = Arc::new(Sphere::new(
        Point3::new(0.5, 0.1, -0.5),
        0.5,
        material_rightmost.clone(),
    ));
    let cube = Arc::new(Cube::new_oriented(
        Point3::new(2., 0.0, -2.),
        0.3,
        &Basis::new_orthonormal(),
        material_cube.clone(),
    ));

    let Cylinder { tube, top, bottom } = Cylinder::new(
        Point3::new(0.2, -0.3, -1.),
        Direction::new(-0.2, 3., -0.4),
        0.3,
        material_left.clone(),
        glass.clone(),
        glass.clone(),
    );

    let mut world = HittableList::new();
    world.add(ground);
    world.add(cube);
    world.add(left);
    world.add(right);
    world.add(rightmost);
    world.add(tube);
    world.add(top);
    world.add(bottom);

    world
}
