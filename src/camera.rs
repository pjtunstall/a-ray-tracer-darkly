use std::io::{self, Write};

use rand::{Rng, rngs::ThreadRng};

use crate::{
    color::Color,
    examples, file,
    hittable::Hittable,
    image::Image,
    interval::Interval,
    progress,
    ray::Ray,
    vec3::{Direction, Point3},
    viewport::Viewport,
};

pub struct Camera {
    // aspect_ratio: f64, // image_width / image_height
    image: Image,
    // viewport: Viewport,
    center: Point3,
    center_of_top_left_pixel: Point3,
    pixel_du: Direction, // offset to pixel on the right
    pixel_dv: Direction, // offset to pixel below
    // focal_length: f64,
    rng: ThreadRng,
    max_depth: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let focal_length = 1.0;
        let center = Point3::new(0., 0., 0.);
        let image = Image::new(image_width, aspect_ratio);
        let viewport = Viewport::new(2.0, &image);

        let pixel_du = viewport.u / image.width as f64;
        let pixel_dv = viewport.v / image.height as f64;
        let viewport_top_left_corner =
            center - Direction::new(0., 0., focal_length) - viewport.u / 2. - viewport.v / 2.;
        let center_of_top_left_pixel = viewport_top_left_corner + 0.5 * (pixel_du + pixel_dv);

        Camera {
            // aspect_ratio,
            image,
            // viewport,
            pixel_du,
            pixel_dv,
            center_of_top_left_pixel,
            center: Point3::new(0., 0., 0.),
            // focal_length: 1.0,
            rng: rand::rng(),
            max_depth: 10,
        }
    }

    // image_name without extension
    pub fn render<T: Hittable>(
        &mut self,
        world: &T,
        image_name: &str,
        samples_per_pixel: usize,
    ) -> io::Result<()> {
        let image_width = self.image.width;
        let image_height = self.image.height;

        let mut writer = file::writer(image_name)?;
        writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

        for i in 0..image_height {
            progress::show(i as usize, image_height as usize, "Rendering");
            for j in 0..image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&ray, world, self.max_depth);
                }

                (pixel_color / samples_per_pixel as f64)
                    .write(&mut writer)
                    .expect("Failed to write pixel color");
            }
        }

        progress::show(image_height as usize, image_height as usize, "Rendering");
        println!();

        Ok(())
    }

    fn ray_color<T: Hittable>(&mut self, ray: &Ray, world: &T, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }
        if let Some(record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) =
                record.material.scatter(&ray, &record.point, &record.normal)
            {
                attenuation * self.ray_color(&scattered, world, depth - 1)
            } else {
                Color::new(0., 0., 0.)
            }
        } else {
            examples::sky(ray)
        }
    }

    fn get_ray(&mut self, i: u32, j: u32) -> Ray {
        // Construct a camera ray from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.center_of_top_left_pixel
            + ((j as f64 + offset.x) * self.pixel_du)
            + ((i as f64 + offset.y) * self.pixel_dv);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&mut self) -> Direction {
        Direction::new(
            self.rng.random_range(-0.5..0.5),
            self.rng.random_range(-0.5..0.5),
            0.,
        )
    }
}
