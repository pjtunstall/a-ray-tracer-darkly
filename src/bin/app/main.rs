mod examples;
mod trace;

use std::io;

fn main() -> io::Result<()> {
    examples::_1::gradient()?;

    trace!(_2, sky)?;
    trace!(_3, red_sphere)?;
    trace!(_4, color_map_sphere)?;

    Ok(())
}
