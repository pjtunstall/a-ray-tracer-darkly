use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    // // Increase `samples_per_pixel` and `max_depth` for a higher quality image; reduce them for speed.
    // examples::book_1(50, 50)?;

    // // // Uncomment to render the cover illustration of Ray Tracing in One Weekend, which is bigger than the others and so takes longer. The authors use 500 sampes per pixel for a high-quality image, which will take, as they warn, "quite a while".
    // // examples::random_spheres::render(50, 10)?;

    // examples::cubes::render(50, 10)?;

    examples::audit::sphere::a_scene_with_a_sphere(10, 10, 800)?;

    Ok(())
}
