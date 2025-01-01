use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::AbsDiffEq;
use image::Rgb;

use crate::vec::Vec3;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Color(Vec3);

#[allow(dead_code)]
impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3(r, g, b))
    }

    pub const fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub const fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub const fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub const fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn random() -> Self {
        Self(Vec3::random())
    }

    pub const fn r(&self) -> f64 {
        self.0 .0
    }

    pub const fn g(&self) -> f64 {
        self.0 .1
    }

    pub const fn b(&self) -> f64 {
        self.0 .2
    }

    pub fn clamped(&self) -> Self {
        Self(Vec3(
            self.0 .0.clamp(0.0, 1.0),
            self.0 .1.clamp(0.0, 1.0),
            self.0 .2.clamp(0.0, 1.0),
        ))
    }

    pub fn clamp(&mut self) {
        self.0 .0 = self.0 .0.clamp(0.0, 1.0);
        self.0 .1 = self.0 .1.clamp(0.0, 1.0);
        self.0 .2 = self.0 .2.clamp(0.0, 1.0);
    }

    pub fn to_gamma_space(self) -> Self {
        Self(Vec3(self.0 .0.sqrt(), self.0 .1.sqrt(), self.0 .2.sqrt()))
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        [
            (self.r() * 255.0).round() as u8,
            (self.g() * 255.0).round() as u8,
            (self.b() * 255.0).round() as u8,
        ]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Color {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
            f64::from(value.0) / 255.0,
            f64::from(value.1) / 255.0,
            f64::from(value.2) / 255.0,
        )
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<Color> for Rgb<u8> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(value: Color) -> Self {
        let clamped = value.clamped();
        Self([
            (clamped.0 .0 * 255.0) as u8,
            (clamped.0 .1 * 255.0) as u8,
            (clamped.0 .2 * 255.0) as u8,
        ])
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        Self(Vec3(
            f64::from(value.0[0]) / 255.0,
            f64::from(value.0[1]) / 255.0,
            f64::from(value.0[2]) / 255.0,
        ))
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.r(), &other.r(), epsilon)
            && f64::abs_diff_eq(&self.g(), &other.g(), epsilon)
            && f64::abs_diff_eq(&self.b(), &other.b(), epsilon)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::Color;

    #[test]
    fn display() {
        let color = Color::from((0, 127, 255));

        assert_eq!(color.to_string(), "0 127 255");
    }

    #[test]
    fn from_integer_color() {
        let color = Color::from((0, 127, 255));

        assert_abs_diff_eq!(color, Color::new(0.0, 0.5, 1.0), epsilon = 0.01);
    }

    #[test]
    fn clamping() {
        let mut c1 = Color::new(1.5, -0.7, 0.4);

        assert_abs_diff_eq!(c1.clamped(), Color::new(1.0, 0.0, 0.4));
        c1.clamp();
        assert_abs_diff_eq!(c1, Color::new(1.0, 0.0, 0.4));
    }

    #[test]
    fn addition() {
        let mut c1 = Color::new(1.0, 0.5, 0.1);
        let c2 = Color::new(0.5, 1.0, 0.1);

        assert_abs_diff_eq!(c1 + c2, Color::new(1.5, 1.5, 0.2));
        c1 += c2;
        assert_abs_diff_eq!(c1, Color::new(1.5, 1.5, 0.2));
    }

    #[test]
    fn subtraction() {
        let mut c1 = Color::new(0.5, 1.0, 0.1);
        let c2 = Color::new(1.0, 0.5, 0.1);

        assert_abs_diff_eq!(c1 - c2, Color::new(-0.5, 0.5, 0.0));
        c1 -= c2;
        assert_abs_diff_eq!(c1, Color::new(-0.5, 0.5, 0.0));
    }

    #[test]
    fn multiplication() {
        let mut c1 = Color::new(1.0, 0.5, 0.1);
        let c2 = Color::new(0.5, 1.0, 0.1);
        let t = 2.0;

        assert_abs_diff_eq!(c1 * c2, Color::new(0.5, 0.5, 0.01));
        c1 *= c2;
        assert_abs_diff_eq!(c1, Color::new(0.5, 0.5, 0.01));
        assert_abs_diff_eq!(c1 * t, Color::new(1.0, 1.0, 0.02));
        c1 *= t;
        assert_abs_diff_eq!(c1, Color::new(1.0, 1.0, 0.02));
    }

    #[test]
    fn division() {
        let mut c1 = Color::new(1.0, 0.5, 0.1);
        let t = 2.0;

        assert_abs_diff_eq!(c1 / t, Color::new(0.5, 0.25, 0.05));
        c1 /= t;
        assert_abs_diff_eq!(c1, Color::new(0.5, 0.25, 0.05));
    }
}
