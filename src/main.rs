use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    examples::book_1(10, 10)?;

    // // Uncomment to render the cover illustration for Book 1, which is bigger than the others and so takes longer. They use 500 sampes per pixel, which may take some hours.
    // examples::random_spheres::render(10, 10)?;

    Ok(())
}
