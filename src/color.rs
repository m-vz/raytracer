use std::fmt::{Display, Formatter};

use float_cmp::{ApproxEq, F64Margin};

use crate::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Color(Vec3);

#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3(r, g, b))
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn r(&self) -> f64 {
        self.0 .0
    }

    pub fn g(&self) -> f64 {
        self.0 .1
    }

    pub fn b(&self) -> f64 {
        self.0 .2
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.r() * 255.0) as u8,
            (self.g() * 255.0) as u8,
            (self.b() * 255.0) as u8
        )
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new(
            value.0 as f64 / 255.0,
            value.1 as f64 / 255.0,
            value.2 as f64 / 255.0,
        )
    }
}

impl ApproxEq for Color {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.r().approx_eq(other.r(), margin)
            && self.g().approx_eq(other.g(), margin)
            && self.b().approx_eq(other.b(), margin)
    }

    fn approx_ne<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.r().approx_ne(other.r(), margin)
            && self.g().approx_ne(other.g(), margin)
            && self.b().approx_ne(other.b(), margin)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::Color;

    #[test]
    fn display() {
        let color = Color::from((0, 127, 255));

        assert_eq!(color.to_string(), "0 127 255");
    }

    #[test]
    fn from_integer_color() {
        let color = Color::from((0, 127, 255));

        assert_approx_eq!(Color, color, Color::new(0.0, 0.5, 1.0), epsilon = 0.01);
    }
}
