use rt::{color::Color, ray::Ray, vec3::Point3};

pub fn red_sphere(r: &Ray) -> Color {
    if is_hit_sphere(Point3::new(0., 0., -1.), 0.5, r) {
        Color::new(1., 0., 0.)
    } else {
        let unit_direction = r.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.);
        (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
    }
}

fn is_hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let origin_to_center = center - r.origin;
    let a = r.direction.dot(&r.direction);
    let b = -2.0 * r.direction.dot(&origin_to_center);
    let c = origin_to_center.dot(&origin_to_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}
