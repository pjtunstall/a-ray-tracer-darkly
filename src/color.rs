use std::{
    io::Write,
    ops::{Add, Div, Mul, Sub},
};

use crate::interval::Interval;

#[derive(Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        linear_component.sqrt()
    } else {
        0.
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn write<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let r = linear_to_gamma(self.r);
        let g = linear_to_gamma(self.g);
        let b = linear_to_gamma(self.b);

        let intensity = Interval::new(0.0, 0.999);
        let r = (256.0 * intensity.clamp(r)) as u8;
        let g = (256.0 * intensity.clamp(g)) as u8;
        let b = (256.0 * intensity.clamp(b)) as u8;

        writeln!(out, "{} {} {}", r, g, b)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        let r = self.r + rhs.r;
        let g = self.g + rhs.g;
        let b = self.b + rhs.b;

        Color::new(r, g, b)
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        let r = self.r - rhs.r;
        let g = self.g - rhs.g;
        let b = self.b - rhs.b;

        Color::new(r, g, b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, t: f64) -> Color {
        Color::new(self.r * t, self.g * t, self.b * t)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self * rhs.r, self * rhs.g * rhs.g, self * rhs.b)
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, t: f64) -> Color {
        self * (1.0 / t)
    }
}

pub fn lerp(color_1: Color, color_2: Color, a: f64) -> Color {
    (1.0 - a) * color_1 + a * color_2
}
