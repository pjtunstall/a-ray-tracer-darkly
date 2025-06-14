use crate::vec3::{DirectionType, PointType, Vec3};

pub struct Ray {
    origin: Vec3<PointType>,
    direction: Vec3<DirectionType>,
}

impl Ray {
    pub fn new(origin: Vec3<PointType>, direction: Vec3<DirectionType>) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3<PointType> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> &Vec3<PointType> {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3<DirectionType> {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    fn is_expected(ray: &Ray, t: f64, expected: Vec3<PointType>) {
        assert!(vec3::approx_eq(ray.at(t), expected, 0.0001));
    }

    #[test]
    fn test_at() {
        let ray = Ray::new(
            vec3::Point3::new(2.0, 3.0, 4.0),
            vec3::Direction::new(1.0, 0.0, 0.0),
        );
        is_expected(&ray, 0.0, vec3::Point3::new(2.0, 3.0, 4.0));
        is_expected(&ray, 1.0, vec3::Point3::new(3.0, 3.0, 4.0));
        is_expected(&ray, -1.0, vec3::Point3::new(1.0, 3.0, 4.0));
        is_expected(&ray, 2.5, vec3::Point3::new(4.5, 3.0, 4.0));
    }
}
