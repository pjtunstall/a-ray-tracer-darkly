use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use rt::{
    image::Image,
    progress,
    vec3::{Point3, color::Color},
    viewport::Viewport,
};

fn main() -> io::Result<()> {
    first_example()?;

    let eye_point = Point3::new(0., 0., 0.);
    let focal_length = 1.0;

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

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    for i in 0..image_height {
        progress::show(i, image_height, "Rendering");
        for j in 0..image_width {
            let r = j as f64 / (image_width - 1) as f64;
            let g = i as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            pixel_color
                .write(&mut writer)
                .expect("Failed to write pixel color");
        }
    }

    progress::show(image_height, image_height, "Rendering");
    println!();

    Ok(())
}
