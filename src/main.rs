mod examples;

use std::io;

fn main() -> io::Result<()> {
    examples::_1::demo_ppm()?;

    trace!(_2, simple_gradient)?;
    trace!(_3, red_sphere)?;

    Ok(())
}
