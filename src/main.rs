use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    // For the cover illustration, they use 500 sampes per pixel, but this may take some hours.
    examples::book_1(10)?;
    Ok(())
}
