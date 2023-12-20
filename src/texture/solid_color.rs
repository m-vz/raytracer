use crate::color::Color;
use crate::texture::Texture;
use crate::vec::Vec3;

pub struct SolidColor(pub Color);

impl SolidColor {
    pub fn black() -> Self {
        Self(Color::black())
    }

    pub fn white() -> Self {
        Self(Color::white())
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Color {
        self.0
    }
}
