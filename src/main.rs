use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    examples::book_1(10)?;
    Ok(())
}
