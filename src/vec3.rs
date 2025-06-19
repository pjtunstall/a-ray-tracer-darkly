use std::{
    // f64::consts::PI,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rand::Rng;

use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointType;

pub type Point3 = Vec3<PointType>;
pub type Direction = Vec3<DirectionType>;

pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Vec3::new(x, y, z)
}

pub fn direction(x: f64, y: f64, z: f64) -> Direction {
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

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.x.abs() < epsilon && self.y < epsilon && self.z < epsilon
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

impl Add for Direction {
    type Output = Direction;
    fn add(self, rhs: Direction) -> Direction {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Direction {
    type Output = Direction;
    fn sub(self, rhs: Direction) -> Direction {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add<Direction> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Direction) -> Point3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Direction> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Direction) -> Point3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Direction;
    fn sub(self, rhs: Point3) -> Direction {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Direction {
    pub fn random_unit() -> Self {
        loop {
            let v = Self::random(-1., 1.);
            let len_sq = v.length_squared();
            if 1e-160 < len_sq && len_sq <= 1. {
                return v / f64::sqrt(len_sq);
            }
        }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();
        let a = rng.random_range(min..max);
        let b = rng.random_range(min..max);
        let c = rng.random_range(min..max);
        Direction::new(a, b, c)
    }

    pub fn to_color(&self) -> Color {
        // Map each component (necessarily in the range [-1, 1] because `n` is a unit vector), to the range [0, 1].
        0.5 * Color::new(self.x + 1., self.y + 1., self.z + 1.)
    }

    pub fn reflect(&self, normal: &Direction) -> Direction {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Direction, refraction_index: f64) -> Direction {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = refraction_index * (*self + cos_theta * *normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *normal;
        r_out_perp + r_out_parallel
    }
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
        assert!(approx_eq(result, Vec3::new(2.0, 4.0, 6.0), f64::EPSILON));
        let result = 2.0 * v;
        assert!(approx_eq(result, Vec3::new(2.0, 4.0, 6.0), f64::EPSILON));
    }

    #[test]
    fn add_direction_to_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p + d;
        assert!(approx_eq(result, Vec3::new(1.5, 1.0, 5.0), f64::EPSILON));
    }

    #[test]
    fn subtract_direction_from_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p - d;
        assert!(approx_eq(result, Vec3::new(0.5, 3.0, 1.0), f64::EPSILON));
    }

    #[test]
    fn subtract_points_gives_direction() {
        let a = Point3::new(3.0, 2.0, 1.0);
        let b = Point3::new(1.0, 1.0, 1.0);
        let result = a - b;
        assert!(approx_eq(result, Vec3::new(2.0, 1.0, 0.0), f64::EPSILON));
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

        assert!((point_unit.length() - 1.0).abs() < f64::EPSILON);
        assert!((dir_unit.length() - 1.0).abs() < f64::EPSILON);
    }
}
