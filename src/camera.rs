use std::io::{self, Write};

use rand::{Rng, rngs::ThreadRng};

use crate::{
    color::Color,
    file,
    hittable::Hittable,
    image::Image,
    interval::Interval,
    progress,
    ray::Ray,
    vec3::{Direction, Point3},
    viewport::Viewport,
};

pub struct Camera {
    image: Image,
    look_from: Point3,
    center_of_top_left_pixel: Point3,
    pixel_du: Direction, // offset to pixel on the right
    pixel_dv: Direction, // offset to pixel below
    rng: ThreadRng,
    max_depth: u32,
    defocus_disk_u: Direction,
    defocus_disk_v: Direction,
    defocus_angle: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        vertical_fov: f64,
        look_from: Point3,
        look_at: Point3,
        up: Direction,
        focus_dist: f64,
        defocus_angle: f64,
        max_depth: u32,
    ) -> Self {
        let w = (look_from - look_at).normalize();
        let v = up.normalize();
        let u = v.cross(&w);

        let h = (vertical_fov / 2.).tan();

        let viewport_height = 2. * h * focus_dist;
        let image = Image::new(image_width, aspect_ratio);
        let viewport = Viewport::new(viewport_height, &image, &u, &v);
        let pixel_du = viewport.u / image.width as f64;
        let pixel_dv = viewport.v / image.height as f64;
        let viewport_top_left_corner =
            look_from - focus_dist * w - viewport.u / 2. - viewport.v / 2.;
        let center_of_top_left_pixel = viewport_top_left_corner + 0.5 * (pixel_du + pixel_dv);

        let defocus_radius = focus_dist * (defocus_angle / 2. as f64).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image,
            pixel_du,
            pixel_dv,
            center_of_top_left_pixel,
            look_from,
            rng: rand::rng(),
            max_depth,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        }
    }

    // Specify `image_name` without extension, thus "example" rather than "example.ppm".
    pub fn render<T: Hittable>(
        &mut self,
        world: &T,
        image_name: &str,
        samples_per_pixel: usize,
        background: fn(&Ray) -> Color,
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
                    pixel_color =
                        pixel_color + self.ray_color(&ray, world, self.max_depth, background);
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

    fn ray_color<T: Hittable>(
        &mut self,
        ray: &Ray,
        world: &T,
        depth: u32,
        background: fn(&Ray) -> Color,
    ) -> Color {
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }
        if let Some(record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = record.material.scatter(
                &ray,
                &record.point,
                &record.normal,
                record.front_face,
                &mut self.rng,
            ) {
                attenuation * self.ray_color(&scattered, world, depth - 1, background)
            } else {
                Color::new(0., 0., 0.)
            }
        } else {
            background(&ray)
        }
    }

    fn get_ray(&mut self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.center_of_top_left_pixel
            + ((j as f64 + offset.x) * self.pixel_du)
            + ((i as f64 + offset.y) * self.pixel_dv);

        let ray_origin = if self.defocus_angle <= 0. {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
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

    fn defocus_disk_sample(&mut self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Direction::random_in_unit_disk(&mut self.rng);
        self.look_from + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
