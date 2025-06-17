use rt::{color::Color, ray::Ray, vec3::Point3};

pub fn color_map_sphere(r: &Ray) -> Color {
    // The center is at -1 because the negative z-axis points into the viewport.
    let center = Point3::new(0., 0., -1.);
    let t = hit_sphere(center, 0.5, r);
    if t > 0. {
        let n = (r.at(t) - Point3::new(0., 0., -1.)).normalize();
        return 0.5 * Color::new(n.x + 1., n.y + 1., n.z + 1.);
    }
    let unit_direction = r.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let origin_to_center = center - r.origin;
    let a = r.direction.dot(&r.direction);
    let b = -2.0 * r.direction.dot(&origin_to_center);
    let c = origin_to_center.dot(&origin_to_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
        -1.
    } else {
        (-b - discriminant.sqrt()) / (2. * a)
    }
}
