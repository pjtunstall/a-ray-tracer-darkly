use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    // // Increase `samples_per_pixel` and `max_depth` for a higher quality image; reduce them for speed.
    // examples::book_1(50, 50)?;

    // // // Uncomment to render the cover illustration of Ray Tracing in One Weekend, which is bigger than the others and so takes longer. The authors use 500 sampes per pixel for a high-quality image, which will take, as they warn, "quite a while".
    // // examples::book::random_spheres::render(50, 500)?;

    // examples::other::cubes::render(10, 10)?;
    // examples::other::combo::render(10, 10, 800)?;

    let max_depth = 50;
    let samples_per_pixel = 50;
    let image_width = 800;
    examples::audit::sphere_scene::render(max_depth, samples_per_pixel, image_width)?;
    examples::audit::cube_and_plane::render(max_depth, samples_per_pixel, image_width)?;
    examples::audit::various_x2::render(max_depth, samples_per_pixel, image_width)?;

    Ok(())
}
