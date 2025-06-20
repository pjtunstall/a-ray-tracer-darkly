pub mod dull_metal;
pub mod glass;
pub mod lambertian;
pub mod shiny_metal;

use crate::{
    color::{self, Color},
    ray::Ray,
};

pub fn sky(ray: &Ray) -> Color {
    let color_1 = Color::new(1.0, 1.0, 1.0);
    let color_2 = Color::new(0.5, 0.7, 1.0);
    let a = 0.5 * (ray.direction.normalize().y + 1.0);
    color::lerp(color_1, color_2, a)
}
