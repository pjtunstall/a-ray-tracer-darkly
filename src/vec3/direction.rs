use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{IntoVec3, Vec3, phantom::DirectionType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction(Vec3<DirectionType>);

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Direction(Vec3::new(x, y, z))
    }
}

impl Add for Direction {
    type Output = Direction;
    fn add(self, rhs: Direction) -> Direction {
        Direction(self.0 + rhs.0)
    }
}

impl Sub for Direction {
    type Output = Direction;
    fn sub(self, rhs: Direction) -> Direction {
        Direction(self.0 - rhs.0)
    }
}

impl Mul<f64> for Direction {
    type Output = Direction;
    fn mul(self, t: f64) -> Direction {
        Direction(self.0 * t)
    }
}

impl Mul<Direction> for f64 {
    type Output = Direction;

    fn mul(self, direction: Direction) -> Direction {
        Direction(direction.0 * self)
    }
}

impl Div<f64> for Direction {
    type Output = Direction;
    fn div(self, t: f64) -> Direction {
        Direction(self.0 / t)
    }
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Direction {
        Direction(-self.0)
    }
}

impl std::ops::Deref for Direction {
    type Target = Vec3<DirectionType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Direction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoVec3<DirectionType> for Direction {
    fn into_inner(self) -> Vec3<DirectionType> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn addition() {
        let d_1 = Direction::new(1.0, -2.0, 0.5);
        let d_2 = Direction::new(-2.0, 3.0, 1.0);
        let result = d_1 + d_2;
        assert!(vec3::approx_eq(
            result.into_inner(),
            Direction::new(-1.0, 1.0, 1.5).into_inner(),
            0.0001
        ));
    }

    #[test]
    fn subtraction() {
        let d_1 = Direction::new(1.0, -2.0, 0.5);
        let d_2 = Direction::new(-2.0, 3.0, 1.0);
        let result = d_1 - d_2;
        assert!(vec3::approx_eq(
            result.into_inner(),
            Direction::new(3.0, -5.0, -0.5).into_inner(),
            0.0001
        ));
    }

    #[test]
    fn scalar_multiplication() {
        let d = Direction::new(1.0, -2.0, 0.5);
        let result_r = d * 2.0;
        let result_l = 2.0 * d;
        assert!(vec3::approx_eq(
            result_r.into_inner(),
            Direction::new(2.0, -4.0, 1.0).into_inner(),
            0.0001
        ));
        assert!(vec3::approx_eq(
            result_l.into_inner(),
            Direction::new(2.0, -4.0, 1.0).into_inner(),
            0.0001
        ));
    }
}
