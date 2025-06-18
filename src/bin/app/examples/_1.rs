use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::MAIN_SEPARATOR,
};

use rt::{color::Color, progress, render};

pub fn gradient() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    render::create_images_dir()?;
    let file = File::create(format!("images{}example_1.ppm", MAIN_SEPARATOR))?;
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
