use std::{
    // f64::consts::PI,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointType;

pub type Point3 = Vec3<PointType>;
pub type Direction = Vec3<DirectionType>;

pub fn point3(x: f64, y: f64, z: f64) -> Vec3<PointType> {
    Vec3::new(x, y, z)
}

pub fn direction(x: f64, y: f64, z: f64) -> Vec3<DirectionType> {
    Vec3::new(x, y, z)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    _marker: PhantomData<T>,
}

impl<T> Vec3<T> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            _marker: PhantomData,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3<T>) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<T: Copy> Vec3<T> {
    pub fn normalize(&self) -> Vec3<T> {
        *self / self.length()
    }
}

impl<T> Neg for Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Vec3<T> {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Mul<f64> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, t: f64) -> Vec3<T> {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl<T> Mul<Vec3<T>> for f64 {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl<T> Div<f64> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, t: f64) -> Vec3<T> {
        self * (1.0 / t)
    }
}

impl Add for Vec3<DirectionType> {
    type Output = Vec3<DirectionType>;
    fn add(self, rhs: Vec3<DirectionType>) -> Vec3<DirectionType> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3<DirectionType> {
    type Output = Vec3<DirectionType>;
    fn sub(self, rhs: Vec3<DirectionType>) -> Vec3<DirectionType> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add<Vec3<DirectionType>> for Vec3<PointType> {
    type Output = Vec3<PointType>;
    fn add(self, rhs: Vec3<DirectionType>) -> Vec3<PointType> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3<DirectionType>> for Vec3<PointType> {
    type Output = Vec3<PointType>;
    fn sub(self, rhs: Vec3<DirectionType>) -> Vec3<PointType> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vec3<PointType>> for Vec3<PointType> {
    type Output = Vec3<DirectionType>;
    fn sub(self, rhs: Vec3<PointType>) -> Vec3<DirectionType> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Direction {
    pub fn to_color(&self) -> Color {
        // Map each component (necessarily in the range [-1, 1] because `n` is a unit vector), to the range [0, 1].
        0.5 * Color::new(self.x + 1., self.y + 1., self.z + 1.)
    }

    // pub fn reflect(&self, normal: &Vec3<DirectionType>) -> Vec3<DirectionType> {
    //     *self - 2.0 * self.dot(normal) * *normal
    // }

    // pub fn refract(
    //     &self,
    //     normal: &Vec3<DirectionType>,
    //     etai_over_etat: f64,
    // ) -> Vec3<DirectionType> {
    //     let cos_theta = (-*self).dot(normal).min(1.0);
    //     let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
    //     let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *normal;
    //     r_out_perp + r_out_parallel
    // }
}

pub fn approx_eq<T>(a: Vec3<T>, b: Vec3<T>, epsilon: f64) -> bool {
    (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_for_direction() {
        let v = direction(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert!(approx_eq(result, Vec3::new(2.0, 4.0, 6.0), 0.0001));
        let result = 2.0 * v;
        assert!(approx_eq(result, Vec3::new(2.0, 4.0, 6.0), 0.0001));
    }

    #[test]
    fn add_direction_to_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p + d;
        assert!(approx_eq(result, Vec3::new(1.5, 1.0, 5.0), 0.0001));
    }

    #[test]
    fn subtract_direction_from_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p - d;
        assert!(approx_eq(result, Vec3::new(0.5, 3.0, 1.0), 0.0001));
    }

    #[test]
    fn subtract_points_gives_direction() {
        let a = Point3::new(3.0, 2.0, 1.0);
        let b = Point3::new(1.0, 1.0, 1.0);
        let result = a - b;
        assert!(approx_eq(result, Vec3::new(2.0, 1.0, 0.0), 0.0001));
    }

    #[test]
    fn point_equality() {
        let a = Point3::new(1.0, 2.0, 3.0);
        let b = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_normalize_for_markers() {
        let point = Point3::new(3.0, 0.0, 4.0);
        let dir = Direction::new(0.0, 5.0, 12.0);

        let point_unit = point.normalize();
        let dir_unit = dir.normalize();

        assert!((point_unit.length() - 1.0).abs() < 1e-6);
        assert!((dir_unit.length() - 1.0).abs() < 1e-6);
    }
}
