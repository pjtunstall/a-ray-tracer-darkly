use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    _marker: PhantomData<T>,
}

// Type markers.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PointType;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct DirectionType;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct ColorType;

// Type aliases.
pub type Point3 = Vec3<PointType>;
pub type Color = Vec3<ColorType>;
pub type Direction = Vec3<DirectionType>;

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

    // pub fn unit_vector(&self) -> Vec3<T> {
    //     Vec3::new(self.x, self.y, self.z) / self.length()
    // }
}

impl<T: Copy + Clone> Vec3<T> {
    pub fn unit_vector(&self) -> Vec3<T> {
        *self / self.length()
    }
}

impl<T> Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

macro_rules! impl_sub_same_type {
    ($T:ty) => {
        impl Sub for Vec3<$T> {
            type Output = Vec3<$T>;
            fn sub(self, rhs: Self) -> Self::Output {
                Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }
    };
}

impl_sub_same_type!(ColorType);
impl_sub_same_type!(DirectionType);

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

impl<T> Div<f64> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, t: f64) -> Vec3<T> {
        self * (1.0 / t)
    }
}

impl Add<Vec3<DirectionType>> for Vec3<PointType> {
    type Output = Vec3<PointType>;

    fn add(self, rhs: Vec3<DirectionType>) -> Vec3<PointType> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3<PointType>> for Vec3<PointType> {
    type Output = Vec3<DirectionType>;

    fn sub(self, rhs: Vec3<PointType>) -> Vec3<DirectionType> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

pub fn approx_eq<T>(a: Vec3<T>, b: Vec3<T>, epsilon: f64) -> bool {
    (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_direction_to_point() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.5, -1.0, 2.0);
        let result = p + d;
        assert!(approx_eq(result, Point3::new(1.5, 1.0, 5.0), 0.0001));
    }

    #[test]
    fn subtract_points_gives_direction() {
        let a = Point3::new(3.0, 2.0, 1.0);
        let b = Point3::new(1.0, 1.0, 1.0);
        let result = a - b;
        assert!(approx_eq(result, Direction::new(2.0, 1.0, 0.0), 0.0001));
    }

    #[test]
    fn color_addition() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.2, 0.3, 0.4);
        let result = a + b;
        assert!(approx_eq(result, Color::new(0.3, 0.5, 0.7), 0.0001));
    }

    #[test]
    fn direction_scalar_multiplication() {
        let d = Direction::new(1.0, -2.0, 0.5);
        let result = d * 2.0;
        assert!(approx_eq(result, Direction::new(2.0, -4.0, 1.0), 0.0001));
    }

    #[test]
    fn point_equality() {
        let a = Point3::new(1.0, 2.0, 3.0);
        let b = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_unit_vector_for_markers() {
        let point = Vec3::<PointType>::new(3.0, 0.0, 4.0);
        let dir = Vec3::<DirectionType>::new(0.0, 5.0, 12.0);
        let color = Vec3::<ColorType>::new(0.1, 0.2, 0.2);

        let point_unit = point.unit_vector();
        let dir_unit = dir.unit_vector();
        let color_unit = color.unit_vector();

        assert!((point_unit.length() - 1.0).abs() < 1e-6);
        assert!((dir_unit.length() - 1.0).abs() < 1e-6);
        assert!((color_unit.length() - 1.0).abs() < 1e-6);
    }
}
