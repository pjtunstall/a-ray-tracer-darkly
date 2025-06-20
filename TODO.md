- Refactor examples and camera.
- Parametrize depth and samples per pixel.
- Credit Peter Shirley, Trevor David Black, Steve Hollasch: [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html), originally published in 2018. I used Version 4.0.2, 2025-04-25.
- Note that negative z-axis points into the viewport.
- Change Vec3 to be based on an array.
- Decide if it's worth giving the functions that return random items access to a common rng, created in advance, as I originally did by making them methods `camera`.
- Keep track of the previous hit record material so that the next refraction index can be calculated rather than leaving it to the user of the library to get it right in advance.
- Note reversal of definition of `refraction_index` in `scatter` in `Material` for `Dielectric` from the book.
- Look into using rayon.

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
