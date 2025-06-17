use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::{MAIN_SEPARATOR, Path},
};

use crate::{
    color::Color,
    image::Image,
    progress,
    ray::Ray,
    vec3::{Direction, Point3},
    viewport::Viewport,
};

pub fn render(
    image: &Image,
    viewport: &Viewport,
    image_path: &str,
    calculate_pixel_color: fn(r: &Ray) -> Color,
) -> io::Result<()> {
    let focal_length = 1.0;
    let camera_center = Point3::new(0., 0., 0.);
    let pixel_du = viewport.u / image.width as f64;
    let pixel_dv = viewport.v / image.height as f64;
    let viewport_upper_left =
        camera_center - Direction::new(0., 0., focal_length) - viewport.u / 2. - viewport.v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);

    create_images_dir()?;
    let file = File::create(format!("images{}{}.ppm", MAIN_SEPARATOR, image_path))?;
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

pub fn create_images_dir() -> io::Result<()> {
    let path = Path::new("images");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}
