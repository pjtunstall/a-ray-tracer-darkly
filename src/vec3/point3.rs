use std::ops::{Add, Sub};

use super::{IntoVec3, Vec3, direction::Direction, phantom::PointType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3(Vec3<PointType>);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3::new(x, y, z))
    }
}

impl Add<Direction> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Direction) -> Point3 {
        let lhs = self.0;
        let rhs = rhs.into_inner();
        Point3(Vec3::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z))
    }
}

impl Sub<Point3> for Point3 {
    type Output = Direction;
    fn sub(self, rhs: Point3) -> Direction {
        let lhs = self.0;
        let rhs = rhs.0;
        Direction::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
    }
}

impl std::ops::Deref for Point3 {
    type Target = Vec3<PointType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Point3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoVec3<PointType> for Point3 {
    fn into_inner(self) -> Vec3<PointType> {
        self.0
    }
}
