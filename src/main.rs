use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use rt::{
    color::Color,
    image::Image,
    progress,
    ray::Ray,
    vec3::{Direction, Point3},
    viewport::Viewport,
};

fn main() -> io::Result<()> {
    first_example()?;
    render("example_2", simple_gradient)?;
    render("example_3", red_sphere)?;

    Ok(())
}

pub fn make_image_and_viewport() -> (Image, Viewport) {
    let aspect_ratio = 16.0 / 9.0;
    let image = Image::new(400, aspect_ratio);
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image.width as f64 / image.height as f64);
    let viewport = Viewport::new(viewport_width, viewport_height);
    (image, viewport)
}

fn first_example() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("example_1.ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    for i in 0..image_height {
        progress::show(i, image_height, "Rendering");
        for j in 0..image_width {
            let r = j as f64 / (image_width - 1) as f64;
            let g = i as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let pixel = Color::new(r, g, b);
            pixel
                .write(&mut writer)
                .expect("Failed to write pixel color");
        }
    }

    progress::show(image_height, image_height, "Rendering");
    println!();

    Ok(())
}

fn simple_gradient(r: &Ray) -> Color {
    let color_1 = Color::new(1.0, 1.0, 1.0);
    let color_2 = Color::new(0.5, 0.7, 1.0);
    let a = 0.5 * (r.direction.normalize().y + 1.0);
    lerp(color_1, color_2, a)
}

fn lerp(color_1: Color, color_2: Color, a: f64) -> Color {
    (1.0 - a) * color_1 + a * color_2
}

fn is_hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let origin_to_center = center - r.origin;
    let a = r.direction.dot(&r.direction);
    let b = -2.0 * r.direction.dot(&origin_to_center);
    let c = origin_to_center.dot(&origin_to_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}

fn red_sphere(r: &Ray) -> Color {
    if is_hit_sphere(Point3::new(0., 0., -1.), 0.5, r) {
        Color::new(1., 0., 0.)
    } else {
        let unit_direction = r.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.);
        (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
    }
}

fn render(image_path: &str, calculate_pixel_color: fn(r: &Ray) -> Color) -> io::Result<()> {
    let (image, viewport) = make_image_and_viewport();
    let focal_length = 1.0;
    let camera_center = Point3::new(0., 0., 0.);
    let pixel_du = viewport.u / image.width as f64;
    let pixel_dv = viewport.v / image.height as f64;
    let viewport_upper_left =
        camera_center - Direction::new(0., 0., focal_length) - viewport.u / 2. - viewport.v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);

    let file = File::create(image_path.to_string() + ".ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", image.width, image.height)?;

    for i in 0..image.height {
        progress::show(i as usize, image.height as usize, "Rendering");
        for j in 0..image.width {
            let pixel_center = pixel00_loc + (j as f64 * pixel_du) + (i as f64 * pixel_dv);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            calculate_pixel_color(&r)
                .write(&mut writer)
                .expect("Failed to write pixel color");
        }
    }

    progress::show(image.height as usize, image.height as usize, "Rendering");
    println!();

    Ok(())
}
