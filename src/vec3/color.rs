use std::{
    io::Write,
    ops::{Add, Mul, Sub},
};

use crate::vec3::{Vec3, phantom::ColorType};

use super::IntoVec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(Vec3<ColorType>);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color(Vec3::new(x, y, z))
    }

    pub fn write<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let r = self.x.clamp(0.0, 1.0);
        let g = self.y.clamp(0.0, 1.0);
        let b = self.z.clamp(0.0, 1.0);

        let r_byte = (255.999 * r) as u8;
        let g_byte = (255.999 * g) as u8;
        let b_byte = (255.999 * b) as u8;

        writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color(self.0 - rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, t: f64) -> Color {
        Color(self.0 * t)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color(color.0 * self)
    }
}

impl std::ops::Deref for Color {
    type Target = Vec3<ColorType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoVec3<ColorType> for Color {
    fn into_inner(self) -> Vec3<ColorType> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn addition() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.2, 0.3, 0.4);
        let result = a + b;
        assert!(vec3::approx_eq(
            result.into_inner(),
            Color::new(0.3, 0.5, 0.7).into_inner(),
            0.0001
        ));
    }

    #[test]
    fn subtraction() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.2, 0.3, 0.4);
        let result = a - b;
        assert!(vec3::approx_eq(
            result.into_inner(),
            Color::new(-0.1, -0.1, -0.1).into_inner(),
            0.0001
        ));
    }

    #[test]
    fn scalar_multiplication() {
        let color = Color::new(0.1, 0.2, 0.3);
        let scalar = 2.0;
        let result_l = scalar * color;
        let result_r = color * scalar;
        assert!(vec3::approx_eq(
            result_l.into_inner(),
            Color::new(0.2, 0.4, 0.6).into_inner(),
            0.0001
        ));
        assert!(vec3::approx_eq(
            result_r.into_inner(),
            Color::new(0.2, 0.4, 0.6).into_inner(),
            0.0001
        ));
    }
}
