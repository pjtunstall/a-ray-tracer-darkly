use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::{self, Color},
    hittables::{
        HittableList, cube::Cube, cylinder::Cylinder, plane::Plane, quad::Quad, sphere::Sphere,
    },
    materials::{Lambertian, Light, Metal},
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
        PathBuf::from("audit").join("request"),
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
        look_from: Point3::new(0., 12., 9.),
        look_at: Point3::new(0., 1., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

fn make() -> HittableList {
    let metal_1 = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let metal_2 = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let light_material = Arc::new(Light::new(Color::new(2., 2., 1.)));
    let dark_material = Arc::new(Lambertian::new(Color::new(0.05, 0.05, 0.05)));
    let red_material = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let blue_material = Arc::new(Lambertian::new(Color::new(0.1, 0.1, 0.8)));
    let ground_material = Arc::new(Metal::new(Color::new(0.2, 0.4, 0.4), 0.2));

    let ground = Arc::new(Plane::new(
        Point3::new(0., 0., 0.),
        Direction::new(0., 1., 0.),
        ground_material,
    ));
    let light = Arc::new(Sphere::new(Point3::new(0., 32., 32.), 16., light_material));
    let front = Arc::new(Cube::new_oriented(
        Point3::new(0., 0.5, 0.),
        0.1,
        &Basis::new_orthonormal(),
        blue_material,
    ));
    let back = Arc::new(Cube::new_oriented(
        Point3::new(0., 0.5, -2.),
        0.1,
        &Basis::new_orthonormal(),
        red_material,
    ));
    let center = Arc::new(Sphere::new(Point3::new(0., 0.2, -1.), 0.2, metal_2.clone()));
    let cylinder_right = Arc::new(Cylinder::new(
        Point3::new(1., 0., -1.),
        Direction::new(0., 3., 0.),
        0.2,
        metal_1.clone(),
        metal_1.clone(),
        metal_1.clone(),
    ));
    let cylinder_left = Arc::new(Cylinder::new(
        Point3::new(-1., 0., -1.),
        Direction::new(0., 3., 0.),
        0.2,
        metal_1.clone(),
        metal_1.clone(),
        metal_1.clone(),
    ));

    let quad_00 = Arc::new(Quad::new(
        Point3::new(-1.6, 2., -2.),
        Direction::new(0.3, 0.3, 0.),
        Direction::new(0.3, -0.3, 0.),
        dark_material.clone(),
    ));
    let quad_01 = Arc::new(Quad::new(
        Point3::new(1., 2., -2.),
        Direction::new(0.3, 0.3, 0.),
        Direction::new(0.3, -0.3, 0.),
        dark_material.clone(),
    ));
    let quad_10 = Arc::new(Quad::new(
        Point3::new(-1.6, 2., 0.),
        Direction::new(0.3, 0.3, 0.),
        Direction::new(0.3, -0.3, 0.),
        dark_material.clone(),
    ));
    let quad_11 = Arc::new(Quad::new(
        Point3::new(1., 2., 0.),
        Direction::new(0.3, 0.3, 0.),
        Direction::new(0.3, -0.3, 0.),
        dark_material.clone(),
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(front);
    world.add(back);
    world.add(light);
    world.add(cylinder_left);
    world.add(cylinder_right);
    world.add(quad_00);
    world.add(quad_01);
    world.add(quad_10);
    world.add(quad_11);

    world
}
