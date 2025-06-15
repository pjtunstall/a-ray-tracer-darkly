use std::io::Write;

use super::{ColorType, Vec3};

impl Vec3<ColorType> {
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
