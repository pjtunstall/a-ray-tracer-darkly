use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::{self, Color},
    hittables::{
        HittableList, cylinder::Cylinder, plane::Plane, sphere::Sphere, volumes::ConstantMedium,
    },
    materials::{Dielectric, Lambertian, Light, Metal},
    ray::Ray,
    vec3::{Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make_world();
    let background = sky;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("demo").join("sunset"),
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
        look_from: Point3::new(0., 1., 24.),
        look_at: Point3::new(0., 2., -1.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.,
        vertical_fov_in_degrees: 20.,
    };
    Camera::new(params)
}

fn sky(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    let horizon = Color::new(0.7, 0.5, 0.0);
    let zenith = Color::new(0.05, 0.05, 0.3);
    color::lerp(horizon, zenith, t)
}

pub fn make_world() -> HittableList {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let material_rightmost = Arc::new(Dielectric::new(1.5));
    let material_light = Arc::new(Light::new(Color::new(4., 0.5, 0.)));

    let light = Arc::new(Sphere::new(Point3::new(0., 1.5, -3.0), 0.4, material_light));
    let ground = Arc::new(Plane::new(
        Point3::new(0., -0.5, 0.),
        Direction::new(0., 1., 0.),
        material_ground.clone(),
    ));
    let center = Arc::new(Sphere::new(Point3::new(0., 0., -2.5), 0.5, material_center));
    let left = Arc::new(Sphere::new(Point3::new(-0.5, 0., -3.), 0.5, material_left));
    let right = Arc::new(Sphere::new(Point3::new(1., 0., -1.5), 0.5, material_right));
    let rightmost = Arc::new(Sphere::new(
        Point3::new(1.3, 0., -0.5),
        0.5,
        material_rightmost,
    ));

    let Cylinder { tube, top, bottom } = Cylinder::new(
        Point3::new(-8., 1., -2.9),
        Direction::new(16., 0., 0.),
        0.5,
        material_ground.clone(),
        material_ground.clone(),
        material_ground.clone(),
    );
    let mut cylinder = HittableList::new();
    cylinder.add(tube);
    cylinder.add(top);
    cylinder.add(bottom);
    let cylinder = Arc::new(cylinder);

    let density = 0.3;
    let haze_color = Color::new(0., 0., 0.);
    let haze = Arc::new(ConstantMedium::new(
        cylinder.clone(),
        density,
        haze_color.clone(),
    ));

    let Cylinder { tube, top, bottom } = Cylinder::new(
        Point3::new(-8., 1., -2.9),
        Direction::new(16., 0., 0.),
        0.3,
        material_ground.clone(),
        material_ground.clone(),
        material_ground.clone(),
    );
    let mut cylinder = HittableList::new();
    cylinder.add(tube);
    cylinder.add(top);
    cylinder.add(bottom);
    let inner_cylinder = Arc::new(cylinder);

    let density = 0.5;
    let inner_haze = Arc::new(ConstantMedium::new(
        inner_cylinder.clone(),
        density,
        haze_color,
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    world.add(rightmost);
    world.add(light);
    world.add(haze);
    world.add(inner_haze);

    world
}
