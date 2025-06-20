use std::io;
use std::rc::Rc;

use crate::camera;
use crate::color::Color;
use crate::examples;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Direction, Point3};

pub fn make() -> io::Result<()> {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + rand::random::<f64>(),
            );
            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5..1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material_1.clone(),
    )));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        material_2.clone(),
    )));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        material_3.clone(),
    )));

    let mut camera = camera::Camera::new(
        16. / 9.,
        1200,
        20.,
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Direction::new(0., 1., 0.),
        10.,
        0.6,
    );
    camera.render(&world, "random_spheres", 10, examples::sky::color)?;

    Ok(())
}
