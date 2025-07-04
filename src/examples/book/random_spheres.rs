use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    examples,
    hittables::{HittableList, sphere::Sphere},
    materials::{Dielectric, Lambertian, Metal},
    vec3::{Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize) -> io::Result<()> {
    let world = make();

    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        up: Direction::new(0., 1., 0.),
        focal_distance: 10.,
        defocus_angle_in_degrees: 0.6,
        vertical_fov_in_degrees: 20.,
    };
    let camera = Camera::new(params);

    camera.render(
        &world,
        PathBuf::from("book").join("random_spheres"),
        max_depth,
        samples_per_pixel,
        examples::book::sky::color,
        1.0,
    )?;
    Ok(())
}

fn make() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
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
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5..1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material_1.clone(),
    )));

    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        material_2.clone(),
    )));

    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        material_3.clone(),
    )));

    world
}
