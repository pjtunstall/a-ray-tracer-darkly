use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use rt::{
    image::Image,
    progress,
    vec3::{self, Point3},
    viewport::Viewport,
};

fn main() -> io::Result<()> {
    first_example()?;

    let camera_center = Point3::new(0., 0., 0.);
    let _focal_length = 1.0;

    let (image, viewport) = make_image_and_viewport();
    let _pixel_du = viewport.u / image.width as f64;
    let _pixel_dv = viewport.v / image.height as f64;

    let _viewport_upper_left = camera_center - viewport.u / 2. - viewport.v / 2.;

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

            let pixel_color = vec3::color(r, g, b);
            pixel_color
                .write(&mut writer)
                .expect("Failed to write pixel color");
        }
    }

    progress::show(image_height, image_height, "Rendering");
    println!();

    Ok(())
}
