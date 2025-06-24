- Mention that path tracing is the sort of ray tracing this program does and compare it to Whitted ray tracing.
- Credit Peter Shirley, Trevor David Black, Steve Hollasch: [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html), originally published in 2018. I used Version 4.0.2, 2025-04-25. The [series](https://raytracing.github.io/).
- Note that negative z-axis points into the viewport and that the direction of `camera.v` is reversed compared to that of `viewport.v` and its associated vectors: `pixel_dv` etc.
- Keep track of the previous hit record material so that the next refraction index can be calculated rather than leaving it to the user of the library to get it right in advance. Or, for now, just note clearly that this needs to be done.
- Decide how to present the examples, e.g. all as library functions that can be called, and quote how to call them in the docs.
- Redo cube implementation after reading Quadrilaterals chapter.
- Consider which style to use here, and note the alternatives and reason for choosing one of the other. Either way, maybe encapsulate the logic in a general change-of-basis function, naming it and its parameters something suitably general.
- SmallRng -> Rng trait in vec3 to be more flexible?
- See what fields of shapes can be made private.
- Add more tests.
- Make a Vec3 -> ProjectionOperator function in vec3.
- Assert planar span is suitable for planar primitives Quad and Disk.
- Assert radius of Disk and Sphere are not too small; likewise make sure random points in unit disk are big enough. Check I'm asserting re. value or absolute values, depending on the case.
- Dry out code for Plan, Quad, and Disk.
- Allow Disk to be made either from normal or spanning vectors of plane.
- Restructure how shapes are modularized.
- Make fields of Camera and shapes private?

```rust
 /* This function contains a slick way of writing the change of basis more plainly expresssed as follows.
    Direction::new(
            local_dir[0] * self.u.x + local_dir[1] * self.v.x + local_dir[2] * self.w.x,
            local_dir[0] * self.u.y + local_dir[1] * self.v.y + local_dir[2] * self.w.y,
            local_dir[0] * self.u.z + local_dir[1] * self.v.z + local_dir[2] * self.w.z,
        )
     */
    fn direction_to_world(&self, local_dir: &Direction) -> Direction {
        [self.u, self.v, self.w]
            .into_iter()
            .zip(local_dir)
            .map(|(basis, s)| basis * s)
            .reduce(|a, b| a + b)
            .unwrap()
    }
```

Deviations from the book:

- Note reversal of definition of `refraction_index` in `scatter` in `Material` for `Dielectric` from the book.
- Note that I needed to change FOV to 20 degrees on the defocus blurr example, example_7, like the previous example; the book says 10 degrees, but the view in the illustration matches what I get with 20.
- Note that I've changed FOV to 20 degrees for the earlier examples too as the left and right spheres are elongated otherwise. Apparently this is intentional, since that's how they look in the illustrations in the book.

For the audit, make these 800x600 images in advance:

- A scene with a sphere;
- A scene with a flat plane and a cube with lower brightness than in the sphere image;
- A scene with one of each of all the objects (one cube, one sphere, one cylinder and one flat plane);
- A scene like the previous one, but with the camera in another position (thus generating the same image from a different perspective).

Documentation:

    Explanation on the features of your ray tracer
    Code examples and explanations on how to:
        create an instance of each object (a sphere, a cube, a flat plane and a cylinder)
        change the brightness
        change the camera position and angle

As bonus for this project you can implement:

    Textures to the surfaces of the objects
    Reflection and refraction effects on the objects (make them shiny or reflective)
    Add particles
    Add fluids

Consider putting your bonuses behind command-line flags to achieve a reasonable performance standard defined above. For example, to render textures on your image, you can use a flag -t.

-

Note the effect of the two parameters:

1. samples_per_pixel:

What it controls: How many rays are fired per pixel.

Effect: More samples reduce noise and improve image stability (especially for soft shadows, glossy reflections, and indirect lighting).

Analogy: Like taking multiple noisy photos of the same scene and averaging them — the image gets smoother and clearer.

Impact: Major visual improvement; easily visible.

2. max_depth:

What it controls: How many bounces each ray can take before termination.

Effect: Controls how deep rays can go into light transport — especially for indirect illumination, caustics, reflections, and refractions.

Analogy: Like limiting how far light can bounce around a room — fewer bounces means less global illumination.

Impact:

For scenes dominated by direct lighting, raising the depth beyond 1–2 may not show obvious differences.

In scenes with lots of indirect lighting (e.g. interiors lit through small windows, light bouncing off colored walls), higher max depth contributes to realism — deeper soft shadows, color bleeding, subtle ambient effects.
