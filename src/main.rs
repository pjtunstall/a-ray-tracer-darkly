use std::io;

use rt::examples;

fn main() -> io::Result<()> {
    // // Examples from the textbook Ray Tracing in One Weekend. Increase `samples_per_pixel` and `max_depth` for a higher quality image; reduce them for speed.
    // examples::book_1(10, 50)?;

    // // The cover illustration of Ray Tracing in One Weekend, which is bigger than the others and so takes longer. The authors use 500 sampes per pixel for a high-quality image, which will take, as they warn, "quite a while".
    // examples::book::random_spheres::render(50, 500)?;

    // // Some demos I made.
    // examples::demo::basic::render(10, 500, 800)?;
    // examples::demo::combo::render(10, 500, 800)?;
    // examples::demo::sunset::render(10, 500, 800)?;
    // examples::demo::this_floating_world::render(10, 500, 800)?;

    // These are the scenes I created to meet the requirements of the 01Founders project.
    let max_depth = 50;
    let samples_per_pixel = 500;
    let image_width = 800;
    examples::audit::sphere_scene::render(max_depth, samples_per_pixel, image_width)?; // A scene with a sphere.
    examples::audit::cube_and_plane::render(max_depth, samples_per_pixel, image_width)?; // A cube and plane, darker than the previous image.
    examples::audit::various_x2::render(max_depth, samples_per_pixel, image_width)?; // Two scenes from different points of view with a plane, a sphere, a cube. and a cylinder.

    Ok(())
}
