use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3<ColorType>;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct ColorType;

impl Color {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn color_addition() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.2, 0.3, 0.4);
        let result = a + b;
        assert!(vec3::approx_eq(result, Color::new(0.3, 0.5, 0.7), 0.0001));
    }
}
