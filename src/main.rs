use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    // Increase `samples_per_pixel` and `max_depth` for a higher quality image; reduce them for speed.
    examples::book_1(50, 50)?;

    // // Uncomment to render the cover illustration of Ray Tracing in One Weekend, which is bigger than the others and so takes longer. The authors use 500 sampes per pixel for a high-quality image, which will take, as they warn, "quite a while".
    // examples::random_spheres::render(50, 10)?;

    examples::cubes::render(50, 10)?;

    let max_depth = 10;
    let samples_per_pixel = 10;
    let image_width = 800;
    examples::audit::sphere::a_scene_with_a_sphere(max_depth, samples_per_pixel, image_width)?;
    examples::audit::plane_and_cube::cube_and_plane(max_depth, samples_per_pixel, image_width)?;

    Ok(())
}
