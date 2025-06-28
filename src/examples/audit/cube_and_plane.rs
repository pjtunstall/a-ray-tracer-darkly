use std::{io, path::PathBuf, sync::Arc};

use crate::{
    camera::{Camera, CameraParameters},
    color::{self, Color},
    hittables::{HittableList, cube::Cube, plane::Plane},
    materials::Metal,
    ray::Ray,
    vec3::{Basis, Direction, Point3},
};

pub fn render(max_depth: usize, samples_per_pixel: usize, image_width: u32) -> io::Result<()> {
    let world = make_world();
    let background = sky;
    let camera = set_up_camera(image_width);
    camera.render(
        &world,
        PathBuf::from("audit").join("cube_and_plane"),
        max_depth,
        samples_per_pixel,
        background,
        0.5,
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
    let ground_color = Color::new(0.2, 0.4, 0.4);
    let cube_color = Color::new(0.8, 0.2, 0.2);

    let ground_material = Arc::new(Metal::new(ground_color, 0.2));
    let cube_material = Arc::new(Metal::new(cube_color, 0.1));

    let ground = Arc::new(Plane::new(
        Point3::new(0., -0.5, 0.),
        Direction::new(0., 1., 0.),
        ground_material,
    ));

    let cube = Arc::new(Cube::new_oriented(
        Point3::new(0.0, 0., -5.),
        0.3,
        &Basis::new_orthonormal(),
        cube_material,
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(cube);

    world
}
