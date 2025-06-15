use crate::vec3::{direction::Direction, point3::Point3};

pub struct Ray {
    origin: Point3,
    direction: Direction,
}

impl Ray {
    pub fn new(origin: Point3, direction: Direction) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::{self, IntoVec3, direction::Direction, point3::Point3};

    fn is_expected(ray: &Ray, t: f64, expected: Point3) {
        assert!(vec3::approx_eq(
            ray.at(t).into_inner(),
            expected.into_inner(),
            0.0001
        ));
    }

    #[test]
    fn test_at() {
        let ray = Ray::new(Point3::new(2.0, 3.0, 4.0), Direction::new(1.0, 0.0, 0.0));
        is_expected(&ray, 0.0, Point3::new(2.0, 3.0, 4.0));
        is_expected(&ray, 1.0, Point3::new(3.0, 3.0, 4.0));
        is_expected(&ray, -1.0, Point3::new(1.0, 3.0, 4.0));
        is_expected(&ray, 2.5, Point3::new(4.5, 3.0, 4.0));
    }
}
