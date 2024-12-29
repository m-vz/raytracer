use crate::color::Color;
use crate::texture::Texture;
use crate::vec::Vec3;

pub struct SolidColor(pub Color);

#[allow(dead_code)]
impl SolidColor {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Color::new(r, g, b))
    }

    pub const fn black() -> Self {
        Self(Color::black())
    }

    pub const fn white() -> Self {
        Self(Color::white())
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Color {
        self.0
    }
}
