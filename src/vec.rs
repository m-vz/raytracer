use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use float_cmp::{ApproxEq, F64Margin};

#[derive(Debug, Copy, Clone)]
struct Vec3(f64, f64, f64);

#[allow(dead_code)]
impl Vec3 {
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn unit() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub fn copy(&self) -> Self {
        Self(self.0, self.1, self.2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalized(&self) -> Self {
        self.copy() / self.len()
    }

    pub fn normalize(&mut self) {
        *self /= self.len();
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl ApproxEq for Vec3 {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.0.approx_eq(other.0, margin)
            && self.1.approx_eq(other.1, margin)
            && self.2.approx_eq(other.2, margin)
    }

    fn approx_ne<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.0.approx_ne(other.0, margin)
            && self.1.approx_ne(other.1, margin)
            && self.2.approx_ne(other.2, margin)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::{approx_eq, assert_approx_eq};

    use super::Vec3;

    #[test]
    fn len() {
        let v = Vec3(1.0, 1.0, 0.0);

        assert_approx_eq!(f64, v.len(), 2f64.sqrt());
        assert_approx_eq!(f64, v.len_sq(), 2.0);
    }

    #[test]
    fn normalize() {
        let mut v = Vec3(2.0, 0.0, 0.0);

        assert!(approx_eq!(Vec3, v.normalized(), Vec3(1.0, 0.0, 0.0)));
        v.normalize();
        assert!(approx_eq!(Vec3, v, Vec3(1.0, 0.0, 0.0)))
    }

    #[test]
    fn addition() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        let t = 2.0;

        assert!(approx_eq!(Vec3, v1 + v2, Vec3(5.0, 7.0, 9.0)));
        v1 += v2;
        assert!(approx_eq!(Vec3, v1, Vec3(5.0, 7.0, 9.0)));
        assert!(approx_eq!(Vec3, v1 + t, Vec3(7.0, 9.0, 11.0)));
        v1 += t;
        assert!(approx_eq!(Vec3, v1, Vec3(7.0, 9.0, 11.0)));
    }

    #[test]
    fn subtraction() {
        let mut v1 = Vec3(4.0, 5.0, 6.0);
        let v2 = Vec3(1.0, 2.0, 3.0);
        let t = 2.0;

        assert!(approx_eq!(Vec3, v1 - v2, Vec3(3.0, 3.0, 3.0)));
        v1 -= v2;
        assert!(approx_eq!(Vec3, v1, Vec3(3.0, 3.0, 3.0)));
        assert!(approx_eq!(Vec3, v1 - t, Vec3(1.0, 1.0, 1.0)));
        v1 -= t;
        assert!(approx_eq!(Vec3, v1, Vec3(1.0, 1.0, 1.0)));
    }

    #[test]
    fn multiplication() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        let t = 2.0;

        assert!(approx_eq!(Vec3, v1 * v2, Vec3(4.0, 10.0, 18.0)));
        v1 *= v2;
        assert!(approx_eq!(Vec3, v1, Vec3(4.0, 10.0, 18.0)));
        assert!(approx_eq!(Vec3, v1 * t, Vec3(8.0, 20.0, 36.0)));
        v1 *= t;
        assert!(approx_eq!(Vec3, v1, Vec3(8.0, 20.0, 36.0)));
    }

    #[test]
    fn division() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let t = 2.0;

        assert!(approx_eq!(Vec3, v1 / t, Vec3(0.5, 1.0, 1.5)));
        v1 /= t;
        assert!(approx_eq!(Vec3, v1, Vec3(0.5, 1.0, 1.5)));
    }

    #[test]
    fn dot() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);

        assert_approx_eq!(f64, v1.dot(&v2), 32.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);

        assert!(approx_eq!(Vec3, v1.cross(&v2), Vec3(-3.0, 6.0, -3.0)));
    }
}
