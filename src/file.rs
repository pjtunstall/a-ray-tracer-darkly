use std::{
    fs::{self, File},
    io::{BufWriter, Result},
    path::{Path, PathBuf},
};

pub fn writer<P: AsRef<Path>>(image_path: P) -> Result<BufWriter<File>> {
    let path = PathBuf::from("images")
        .join(&image_path)
        .with_extension("ppm");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}
