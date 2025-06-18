use std::{
    fs::{self, File},
    io::{self, BufWriter, Result},
    path::{MAIN_SEPARATOR, Path},
};

pub fn create_images_dir() -> io::Result<()> {
    let path = Path::new("images");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn writer(image_path: &str) -> Result<BufWriter<File>> {
    create_images_dir()?;
    let path = format!("images{}{}.ppm", MAIN_SEPARATOR, image_path);
    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}
