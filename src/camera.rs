use std::io::{self, Write};

use crate::{
    color::Color,
    examples, file,
    hittable::{HitRecord, Hittable},
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
        }
    }

    // image_name without extension
    pub fn render<T: Hittable>(&self, world: &T, image_name: &str) -> io::Result<()> {
        let image_width = self.image.width;
        let image_height = self.image.height;
        let center_of_top_left_pixel = self.center_of_top_left_pixel;
        let camera_center = self.center;
        let pixel_du = self.pixel_du;
        let pixel_dv = self.pixel_dv;

        let mut writer = file::writer(image_name)?;
        writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

        for i in 0..image_height {
            progress::show(i as usize, image_height as usize, "Rendering");
            for j in 0..image_width {
                let pixel_center =
                    center_of_top_left_pixel + (j as f64 * pixel_du) + (i as f64 * pixel_dv);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_direction);

                self.ray_color(&ray, world)
                    .write(&mut writer)
                    .expect("Failed to write pixel color");
            }
        }

        progress::show(image_height as usize, image_height as usize, "Rendering");
        println!();

        Ok(())
    }

    fn ray_color<T: Hittable>(&self, ray: &Ray, world: &T) -> Color {
        let mut record = HitRecord::default();
        if world.hit(ray, &Interval::new(0., f64::INFINITY), &mut record) {
            record.normal.to_color()
        } else {
            examples::sky(ray)
        }
    }
}
