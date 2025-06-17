use super::_2;
use rt::{color::Color, ray::Ray, vec3::Point3};

pub fn color_map_sphere(ray: &Ray) -> Color {
    // The center is at -1 because the negative z-axis points into the viewport. But, as these functions stand, a sphere at z = 1 would produce the same image because we're only checking that a real solution to the quadrating equation exists. That allows negative solutions, which correspond to points along the negative of ray.direction, i.e. points behind the camera.
    let center = Point3::new(0., 0., -1.);
    let t = hit_sphere(center, 0.5, ray);

    if t > 0. {
        let n = (ray.at(t) - center).normalize();
        0.5 * Color::new(n.x + 1., n.y + 1., n.z + 1.) // Map each component (necessarily in the range [-1, 1] because `n` is a unit vector), to the range [0, 1].
    } else {
        _2::simple_gradient(ray)
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let origin_to_center = center - ray.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = -2.0 * ray.direction.dot(&origin_to_center);
    let c = origin_to_center.dot(&origin_to_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
        -1.
    } else {
        (-b - discriminant.sqrt()) / (2. * a)
    }
}
