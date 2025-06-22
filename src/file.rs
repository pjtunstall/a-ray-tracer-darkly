use std::{
    fs::{self, File},
    io::{self, BufWriter, Result},
    path::{Path, PathBuf},
};

pub fn create_images_dir() -> io::Result<()> {
    let path = Path::new("images");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn writer<P: AsRef<Path>>(image_path: P) -> Result<BufWriter<File>> {
    create_images_dir()?;

    let path = PathBuf::from("images")
        .join(image_path)
        .with_extension("ppm");

    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}
