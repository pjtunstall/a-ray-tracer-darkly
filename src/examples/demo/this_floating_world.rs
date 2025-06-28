use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::{self, Color},
    hittables::{HittableList, cube::Cube, plane::Plane, sphere::Sphere},
    materials::{Dielectric, Light, Metal},
    ray::Ray,
    vec3::{Basis, Direction, Point3},
};

fn sky(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    let horizon = Color::new(0.8, 0.6, 0.4);
    let zenith = Color::new(0.2, 0.3, 0.5);
    color::lerp(horizon, zenith, t)
}

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make();

    let background = sky;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("demo").join("this_floating_world"),
        max_depth,
        samples_per_pixel,
        background,
        1.,
    )?;

    Ok(())
}

fn set_up_camera(image_width: u32) -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16. / 9.0,
        image_width,
        look_from: Point3::new(0., 6., 9.),
        look_at: Point3::new(0., 1., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 30.,
    };
    Camera::new(params)
}

fn make() -> HittableList {
    let water = Arc::new(Dielectric::new(1.33));
    let metal_1 = Arc::new(Metal::new(Color::new(0.1, 0.2, 0.5), 0.5));
    let glass = Arc::new(Dielectric::new(1.5));
    let metal_2 = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let light_material = Arc::new(Light::new(Color::new(2., 2., 1.)));

    let ground = Arc::new(Plane::new(
        Point3::new(0., -0.5, 0.),
        Direction::new(0., 1., 0.),
        water,
    ));
    let light = Arc::new(Sphere::new(Point3::new(0., 1., -5.), 0.2, light_material));
    let center = Arc::new(Cube::new_oriented(
        Point3::new(0., 0., -1.),
        0.3,
        &Basis::new_orthonormal(),
        glass,
    ));
    let left = Arc::new(Cube::new_oriented(
        Point3::new(-1., 0., -1.),
        0.2,
        &Basis::new_orthonormal(),
        metal_1,
    ));
    let right = Arc::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, metal_2));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    world.add(light);

    world
}
