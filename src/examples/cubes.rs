use std::io;
use std::sync::Arc;

use crate::{
    color::Color,
    cube::Cube,
    examples,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Basis, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize) -> io::Result<()> {
    let world = make();

    let background = examples::sky::color;
    let camera = examples::set_up_camera();
    camera.render(
        &world,
        "example_8",
        max_depth,
        samples_per_pixel,
        background,
    )?;

    Ok(())
}

fn make() -> HittableList {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_right = Arc::new(Dielectric::new(1.5));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Box::new(Cube::new(
        Point3::new(0.0, 0., -1.),
        0.5,
        material_center.clone(),
    ));
    let orientation = Basis::new_orthonormal();
    let left = Box::new(Cube::new_oriented(
        Point3::new(-1., 0., -1.),
        0.2,
        material_left.clone(),
        &orientation,
    ));
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);

    world
}
