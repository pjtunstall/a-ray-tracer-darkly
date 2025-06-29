use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::Color,
    hittables::{
        Hittable, HittableList, cube::Cube, plane::Plane, sphere::Sphere, volumes::ConstantMedium,
    },
    materials::Lambertian,
    ray::Ray,
    vec3::{Basis, Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let camera = set_up_camera(image_width);
    let world = create_world();
    let background = sky;

    let brightness = 1.0;

    camera.render(
        &world,
        PathBuf::from("demo").join("smoke"),
        max_depth,
        samples_per_pixel,
        background,
        brightness,
    )?;

    Ok(())
}

fn sky(_ray: &Ray) -> Color {
    Color::new(0.8, 0.8, 0.9)
}

fn set_up_camera(image_width: u32) -> Camera {
    let params = CameraParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width,
        look_from: Point3::new(0.0, 0.2, 4.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        up: Direction::new(0.0, 1.0, 0.0),
        focal_distance: 10.0,
        defocus_angle_in_degrees: 0.0,
        vertical_fov_in_degrees: 20.0,
    };

    Camera::new(params)
}

fn create_world() -> HittableList {
    let ground = ground();
    let sphere = sphere();
    let smoke_sphere = smoke(sphere, Color::new(1., 0., 0.), 0.8);
    let cube = cube();
    let smoke_cube = smoke(cube, Color::new(0., 1., 0.), 0.999);

    let mut world = HittableList::new();
    world.add(ground);
    world.add(smoke_sphere);
    world.add(smoke_cube);

    world
}

fn ground() -> Arc<Plane> {
    let color = Color::new(0.5, 0.5, 0.5);
    let material = Arc::new(Lambertian::new(color));
    let plane = Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Direction::new(0.0, 1.0, 0.0),
        material,
    );
    let ground = Arc::new(plane);
    ground
}

fn sphere() -> Arc<Sphere> {
    let color = Color::new(0.8, 0.4, 0.4);
    let material = Arc::new(Lambertian::new(color));
    let center = Point3::new(0.0, 0.0, -2.5);
    let radius = 0.5;
    let sphere = Arc::new(Sphere::new(center, radius, material));
    sphere
}

fn cube() -> Arc<Cube> {
    let color = Color::new(6., 0.8, 0.8);
    let material = Arc::new(Lambertian::new(color));
    let center = Point3::new(-0.5, 0.5, -4.0);
    let size = 0.3;
    let orientation = &Basis::new_orthonormal();
    let cube = Arc::new(Cube::new_oriented(center, size, orientation, material));
    cube
}

fn smoke(shell: Arc<dyn Hittable + 'static>, color: Color, density: f64) -> Arc<ConstantMedium> {
    let smoke = Arc::new(ConstantMedium::new(shell, color, density));
    smoke
}
