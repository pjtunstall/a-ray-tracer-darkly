use std::{
    io::Write,
    ops::{Add, Div, Mul, Sub},
};

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn write<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let r = self.r.clamp(0.0, 1.0);
        let g = self.g.clamp(0.0, 1.0);
        let b = self.b.clamp(0.0, 1.0);

        let r_byte = (255.999 * r) as u8;
        let g_byte = (255.999 * g) as u8;
        let b_byte = (255.999 * b) as u8;

        writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
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
