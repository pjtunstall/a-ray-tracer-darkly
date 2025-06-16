use crate::vec3::{Direction, Point3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Direction,
}

impl Ray {
    pub fn new(origin: Point3, direction: Direction) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    fn is_expected(ray: &Ray, t: f64, expected: Point3) {
        assert!(vec3::approx_eq(ray.at(t), expected, 0.0001));
    }

    #[test]
    fn test_at() {
        let ray = Ray::new(vec3::point3(2.0, 3.0, 4.0), vec3::direction(1.0, 0.0, 0.0));
        is_expected(&ray, 0.0, vec3::point3(2.0, 3.0, 4.0));
        is_expected(&ray, 1.0, vec3::point3(3.0, 3.0, 4.0));
        is_expected(&ray, -1.0, vec3::point3(1.0, 3.0, 4.0));
        is_expected(&ray, 2.5, vec3::point3(4.5, 3.0, 4.0));
    }
}
