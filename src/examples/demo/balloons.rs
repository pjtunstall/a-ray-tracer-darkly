use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::{self, Color},
    hittables::{
        HittableList, cylinder::Cylinder, plane::Plane, sphere::Sphere, volumes::ConstantMedium,
    },
    materials::{Dielectric, Lambertian, Metal},
    particles,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make_world();
    let background = sky;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("demo").join("balloons"),
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

fn make_world() -> HittableList {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let material_rightmost = Arc::new(Dielectric::new(1.5));

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

    let outer_cylinder = Arc::new(Cylinder::new(
        Point3::new(-8., 1., -2.9),
        Direction::new(16., 0., 0.),
        0.5,
        material_ground.clone(),
        material_ground.clone(),
        material_ground.clone(),
    ));
    let density = 0.3;
    let haze_color = Color::new(0., 0., 0.);
    let outer_haze = Arc::new(ConstantMedium::new(
        outer_cylinder,
        haze_color.clone(),
        density,
    ));

    let inner_cylinder = Arc::new(Cylinder::new(
        Point3::new(-8., 1., -2.9),
        Direction::new(16., 0., 0.),
        0.3,
        material_ground.clone(),
        material_ground.clone(),
        material_ground.clone(),
    ));
    let density = 0.5;
    let inner_haze = Arc::new(ConstantMedium::new(inner_cylinder, haze_color, density));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    world.add(rightmost);
    world.add(outer_haze);
    world.add(inner_haze);

    let center = Point3::new(0., 5., -3.);
    let swarm_radius = 16.;
    let particle_radius = 0.1;
    let particle_material = Arc::new(Lambertian::new(Color::new(4., 0.5, 0.)));
    let bias = 2.0;
    let sampler = particles::power_center_sampler(bias);
    let size = 180;
    let balloons = particles::swarm(
        center,
        swarm_radius,
        particle_radius,
        particle_material,
        size,
        sampler,
    );
    world.add(Arc::new(balloons));

    world
}
