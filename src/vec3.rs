use std::{
    marker::PhantomData,
    ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    _marker: PhantomData<T>,
}

impl<T> Index<usize> for Vec3<T> {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds"),
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec3<T> {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.x, self.y, self.z])
    }
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

    pub fn random(min: f64, max: f64, rng: &mut SmallRng) -> Self {
        let a = rng.random_range(min..max);
        let b = rng.random_range(min..max);
        let c = rng.random_range(min..max);
        Self::new(a, b, c)
    }

    pub fn random_unit(rng: &mut SmallRng) -> Self {
        loop {
            let v = Self::random(-1., 1., rng);
            let len_sq = v.length_squared();
            if 1e-8 < len_sq && len_sq <= 1. {
                return v / f64::sqrt(len_sq);
            }
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() < 1e-8 && self.y < 1e-8 && self.z < 1e-8
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.x.abs() < epsilon && self.y < epsilon && self.z < epsilon
    }
}

impl<T> Vec3<T> {
    pub fn zip<U>(&self, other: &Vec3<U>) -> impl Iterator<Item = (f64, f64)> {
        std::iter::zip(self.into_iter(), other.into_iter())
    }

    pub fn dot<U>(&self, rhs: &Vec3<U>) -> f64 {
        self.zip(rhs).map(|(a, b)| a * b).sum()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointType;

pub type Point3 = Vec3<PointType>;
pub type Direction = Vec3<DirectionType>;

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

impl Point3 {
    pub fn random_in_unit_disk(rng: &mut SmallRng) -> Point3 {
        loop {
            let a = rng.random_range(-1.0..1.0);
            let b = rng.random_range(-1.0..1.0);
            let p = Point3::new(a, b, 0.);
            if p.length_squared() < 1. {
                return p;
            }
        }
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

impl Direction {
    pub fn to_color(&self) -> Color {
        // Map each component (necessarily in the range [-1, 1] if `n` is a unit vector), to the range [0, 1].
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

pub struct Basis {
    pub x: Direction,
    pub y: Direction,
    pub z: Direction,
}

impl Basis {
    pub fn new(x: Direction, y: Direction, z: Direction) -> Self {
        assert!(
            x.dot(&y.cross(&z)) >= 1e-8,
            "Inavalid basis: vectors not independent enough"
        );
        Basis { x, y, z }
    }

    pub fn new_orthonormal() -> Self {
        let mut rng = rand::rng();
        let mut rng = SmallRng::from_rng(&mut rng);

        let x = Direction::random_unit(&mut rng);

        let y = loop {
            let v = Direction::random_unit(&mut rng);
            let proj = x.dot(&v) * x;
            let candidate = v - proj;
            if candidate.length_squared() > 1e-8 {
                break candidate.normalize();
            }
        };

        let z = x.cross(&y);

        Basis { x, y, z }
    }
}

impl Index<usize> for Basis {
    type Output = Direction;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Basis index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Basis {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds"),
        }
    }
}

impl<'a> IntoIterator for &'a Basis {
    type Item = Direction;
    type IntoIter = std::array::IntoIter<Direction, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.x, self.y, self.z])
    }
}

pub fn approx_eq<T>(a: Vec3<T>, b: Vec3<T>, epsilon: f64) -> bool {
    (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-8;

    #[test]
    fn scalar_multiplication_for_direction() {
        let v = Direction::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert!(approx_eq(result, Vec3::new(2.0, 4.0, 6.0), TOLERANCE));
        let result = 2.0 * v;
        assert!(approx_eq(result, Vec3::new(2.0, 4.0, 6.0), TOLERANCE));
    }

    #[test]
    fn add_direction_to_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p + d;
        assert!(approx_eq(result, Vec3::new(1.5, 1.0, 5.0), TOLERANCE));
    }

    #[test]
    fn subtract_direction_from_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p - d;
        assert!(approx_eq(result, Vec3::new(0.5, 3.0, 1.0), TOLERANCE));
    }

    #[test]
    fn subtract_points_gives_direction() {
        let a = Point3::new(3.0, 2.0, 1.0);
        let b = Point3::new(1.0, 1.0, 1.0);
        let result = a - b;
        assert!(approx_eq(result, Vec3::new(2.0, 1.0, 0.0), TOLERANCE));
    }

    #[test]
    fn test_normalize_for_markers() {
        let point = Point3::new(3.0, 0.0, 4.0);
        let dir = Direction::new(0.0, 5.0, 12.0);

        let point_unit = point.normalize();
        let dir_unit = dir.normalize();

        assert!((point_unit.length() - 1.0).abs() < TOLERANCE);
        assert!((dir_unit.length() - 1.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_new_orthonormal() {
        let basis = Basis::new_orthonormal();
        let x = basis.x;
        let y = basis.y;
        let z = basis.z;
        assert!((x.dot(&y)).abs() < TOLERANCE);
        assert!((x.dot(&z)).abs() < TOLERANCE);
        assert!((y.dot(&z)).abs() < TOLERANCE);
    }
}
