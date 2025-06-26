- SmallRng -> Rng trait in vec3 to be more flexible?
- See what fields of shapes can be made private.
- Add more tests.
- Make a Vec3 -> ProjectionOperator function in vec3.
- Dry out code for Plan, Quad, and Disk. Make a general planar primitive struct or trait.
- Allow Disk to be made either from normal or spanning vectors of plane.
- Make fields of Camera and shapes private?
- Tidy names of items in world in `various.rs`.
- Implement indexing and iteration for `Color`.

Deviations from the book:

- Note reversal of definition of `refraction_index` in `scatter` in `Material` for `Dielectric` from the book.
- Note that I needed to change FOV to 20 degrees on the defocus blurr example, example_7, like the previous example; the book says 10 degrees, but the view in the illustration matches what I get with 20.
- Note that I've changed FOV to 20 degrees for the earlier examples too as the left and right spheres are elongated otherwise. Apparently this is intentional, since that's how they look in the illustrations in the book.

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
