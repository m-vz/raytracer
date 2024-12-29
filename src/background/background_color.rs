use crate::background::Background;
use crate::color::Color;

#[derive(Debug, Copy, Clone, Default)]
pub struct BackgroundColor(pub Color);

#[allow(dead_code)]
impl BackgroundColor {
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

impl Background for BackgroundColor {
    fn background(&self, _ray: &crate::ray::Ray) -> Color {
        self.0
    }
}
