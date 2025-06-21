use std::{
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use rand::{Rng, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;

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

pub struct CameraParameters {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub up: Direction,
    pub focal_distance: f64,
    pub defocus_angle_in_degrees: f64,
    pub vertical_fov_in_degrees: f64,
}

#[derive(Clone)]
pub struct Camera {
    image: Image,
    look_from: Point3,
    center_of_top_left_pixel: Point3,
    pixel_du: Direction, // offset to pixel on the right
    pixel_dv: Direction, // offset to pixel below
    defocus_disk_u: Direction,
    defocus_disk_v: Direction,
    defocus_angle: f64,
}

impl Camera {
    pub fn new(params: CameraParameters) -> Self {
        let CameraParameters {
            aspect_ratio,
            image_width,
            look_from,
            look_at,
            up,
            focal_distance,
            defocus_angle_in_degrees,
            vertical_fov_in_degrees,
        } = params;

        let defocus_angle = defocus_angle_in_degrees.to_radians();
        let vertical_fov = vertical_fov_in_degrees.to_radians();

        let w = (look_from - look_at).normalize();
        let v = up.normalize();
        let u = v.cross(&w);

        let h = (vertical_fov / 2.).tan();

        let viewport_height = 2. * h * focal_distance;
        let image = Image::new(image_width, aspect_ratio);
        let viewport = Viewport::new(viewport_height, &image, &u, &v);
        let pixel_du = viewport.u / image.width as f64;
        let pixel_dv = viewport.v / image.height as f64;
        let viewport_top_left_corner =
            look_from - focal_distance * w - viewport.u / 2. - viewport.v / 2.;
        let center_of_top_left_pixel = viewport_top_left_corner + 0.5 * (pixel_du + pixel_dv);

        let defocus_radius = focal_distance * (defocus_angle / 2. as f64).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image,
            pixel_du,
            pixel_dv,
            center_of_top_left_pixel,
            look_from,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        }
    }

    // Specify `image_name` without extension, thus "example" rather than "example.ppm".
    pub fn render<T: Hittable + std::marker::Send + std::marker::Sync>(
        &self,
        world: &T,
        image_name: &str,
        max_depth: usize,
        samples_per_pixel: usize,
        background: fn(&Ray) -> Color,
    ) -> io::Result<()> {
        let mut writer = file::writer(image_name)?;
        writeln!(
            writer,
            "P3\n{} {}\n255",
            self.image.width, self.image.height
        )?;

        let pixels = self.generate_pixels(world, max_depth, samples_per_pixel, background);

        for row in pixels {
            for pixel_color in row {
                pixel_color.write(&mut writer)?;
            }
        }

        Ok(())
    }

    pub fn generate_pixels<T: Hittable + std::marker::Send + std::marker::Sync>(
        &self,
        world: &T,
        max_depth: usize,
        samples_per_pixel: usize,
        background: fn(&Ray) -> Color,
    ) -> Vec<Vec<Color>> {
        let image_width = self.image.width;
        let image_height = self.image.height;
        let base_seed = 12345u64;
        let camera = Arc::new(self.clone());
        let progress = AtomicUsize::new(0);

        let pixels: Vec<Vec<Color>> = (0..image_height)
            .into_par_iter()
            .map(|i| {
                let mut rng = SmallRng::seed_from_u64(base_seed + i as u64);
                let camera = Arc::clone(&camera);
                let row: Vec<Color> = (0..image_width)
                    .map(|j| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
                            let ray = camera.get_ray(i, j, &mut rng);
                            pixel_color = pixel_color
                                + camera.ray_color(&ray, world, max_depth, background, &mut rng);
                        }
                        pixel_color / samples_per_pixel as f64
                    })
                    .collect();
                let done = progress.fetch_add(1, Ordering::Relaxed);
                progress::show(done + 1, image_height as usize, "Rendering");
                row
            })
            .collect();

        progress::show(image_height as usize, image_height as usize, "Rendering");
        println!();

        pixels
    }

    fn ray_color<T: Hittable>(
        &self,
        ray: &Ray,
        world: &T,
        depth: usize,
        background: fn(&Ray) -> Color,
        rng: &mut SmallRng,
    ) -> Color {
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }
        if let Some(record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) =
                record
                    .material
                    .scatter(&ray, &record.point, &record.normal, record.front_face, rng)
            {
                attenuation * self.ray_color(&scattered, world, depth - 1, background, rng)
            } else {
                Color::new(0., 0., 0.)
            }
        } else {
            background(&ray)
        }
    }

    fn get_ray(&self, i: u32, j: u32, rng: &mut SmallRng) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = sample_square(rng);
        let pixel_sample = self.center_of_top_left_pixel
            + ((j as f64 + offset.x) * self.pixel_du)
            + ((i as f64 + offset.y) * self.pixel_dv);

        let ray_origin = if self.defocus_angle <= 0. {
            self.look_from
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self, rng: &mut SmallRng) -> Point3 {
        let p = Point3::random_in_unit_disk(rng);
        self.look_from + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

fn sample_square(rng: &mut SmallRng) -> Direction {
    Direction::new(rng.random_range(-0.5..0.5), rng.random_range(-0.5..0.5), 0.)
}
