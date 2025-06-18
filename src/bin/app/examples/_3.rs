use super::_2;
use rt::{color::Color, ray::Ray, vec3::Point3};

pub fn red_sphere(ray: &Ray) -> Color {
    // The center is at -1 because the negative z-axis points into the viewport. But, as these functions stand, a sphere at z = 1 would produce the same image because we're only checking that a real solution to the quadrating equation exists. That allows negative solutions, which correspond to points along the negative of ray.direction, i.e. points behind the camera.
    if hit_sphere(Point3::new(0., 0., -1.), 0.5, ray) {
        Color::new(1., 0., 0.)
    } else {
        _2::sky(ray)
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let origin_to_center = center - ray.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = -2.0 * ray.direction.dot(&origin_to_center);
    let c = origin_to_center.dot(&origin_to_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}
