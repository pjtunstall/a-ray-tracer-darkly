# rt

## Context

This is a simple ray tracer. It's one of the 01Founders Rust projects. It implments all the features from the first book in the fantastic [Ray Tracing in One Weekend](https://raytracing.github.io/) series (adapting them from C++ to Rust), and a couple from book two (quad and plane), plus a few extras, in particular a cylinder, as required by 01.

## Usage

If you don't already have Rust installed, [install it](https://www.rust-lang.org/tools/install). Clone this repository:

```sh
git clone https://github.com/pjtunstall/rt
```

Then `cd rt`, and run `cargo run --release` to build and run a program with some examples.

## Features

- Techniques:

  - Antialiasing
  - Gamma correction
  - Defocus blur

- Parameters:

  - Position direction of camera
  - maximum recursion depth
  - samples per pixel

- Shapes:

  - Sphere
  - Plane
  - Quad
  - Tube
  - Cylinder
  - Disk
  - Cube

- Materials:

  - Lambertian (matt)
  - Metal (reflective)
  - Dielectric (reflective and refractive: glass, water, etc.)
