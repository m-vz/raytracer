use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

use float_cmp::{ApproxEq, F64Margin};

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

#[allow(dead_code)]
impl Vec3 {
    pub const fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub const fn unit() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub const fn right() -> Self {
        Self(1.0, 0.0, 0.0)
    }

    pub const fn up() -> Self {
        Self(0.0, 1.0, 0.0)
    }

    pub const fn forward() -> Self {
        Self(0.0, 0.0, -1.0)
    }

    pub fn random() -> Self {
        Self(rand::random(), rand::random(), rand::random())
    }

    pub fn random_in_range(range: Range<f64>) -> Self {
        let scale = range.end - range.start;

        Self(
            scale.mul_add(rand::random::<f64>(), range.start),
            scale.mul_add(rand::random::<f64>(), range.start),
            scale.mul_add(rand::random::<f64>(), range.start),
        )
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_in_range(-1.0..1.0);

            if p.len_sq() < 1.0 {
                return p.normalized();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let unit_vector = Self::random_unit_vector();

        if unit_vector.dot(normal) > 0.0 {
            unit_vector
        } else {
            -unit_vector
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut p = Self::random_in_range(-1.0..1.0);
            p.2 = 0.0;

            if p.len_sq() < 1.0 {
                return p;
            }
        }
    }

    pub fn axis(&self, n: u32) -> f64 {
        match n {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn set_axis(&mut self, n: u32, value: f64) {
        match n {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => panic!("Invalid axis"),
        }
    }

    pub const fn x(&self) -> Self {
        Self(self.0, 0.0, 0.0)
    }

    pub const fn y(&self) -> Self {
        Self(0.0, self.1, 0.0)
    }

    pub const fn z(&self) -> Self {
        Self(0.0, 0.0, self.2)
    }

    pub const fn xy(&self) -> Self {
        Self(self.0, self.1, 0.0)
    }

    pub const fn xz(&self) -> Self {
        Self(self.0, 0.0, self.2)
    }

    pub const fn yz(&self) -> Self {
        Self(0.0, self.1, self.2)
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.0
            .mul_add(self.0, self.1.mul_add(self.1, self.2 * self.2))
    }

    pub fn normalized(&self) -> Self {
        *self / self.len()
    }

    pub fn normalize(&mut self) {
        *self /= self.len();
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0.mul_add(rhs.0, self.1.mul_add(rhs.1, self.2 * rhs.2))
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1.mul_add(rhs.2, -(self.2 * rhs.1)),
            self.2.mul_add(rhs.0, -(self.0 * rhs.2)),
            self.0.mul_add(rhs.1, -(self.1 * rhs.0)),
        )
    }

    pub fn floored(&self) -> Self {
        Self(self.0.floor(), self.1.floor(), self.2.floor())
    }

    pub fn floor(&mut self) {
        self.0 = self.0.floor();
        self.1 = self.1.floor();
        self.2 = self.2.floor();
    }

    pub fn look_at(&self, rhs: &Self) -> Self {
        *rhs - *self
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Self, refraction_ratio: f64) -> Self {
        let cos_theta = -self.dot(normal).min(1.0);
        let orthogonal_part = refraction_ratio * (*self + cos_theta * *normal);
        let parallel_part = -(1.0 - orthogonal_part.len_sq()).abs().sqrt() * *normal;

        orthogonal_part + parallel_part
    }
}

impl Add for Vec3 {
    type Output = Self;

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

impl Sub for Vec3 {
    type Output = Self;

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

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

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
    type Output = Self;

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

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

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
    use float_cmp::assert_approx_eq;

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

        assert_approx_eq!(Vec3, v.normalized(), Vec3(1.0, 0.0, 0.0));
        v.normalize();
        assert_approx_eq!(Vec3, v, Vec3(1.0, 0.0, 0.0))
    }

    #[test]
    fn addition() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);

        assert_approx_eq!(Vec3, v1 + v2, Vec3(5.0, 7.0, 9.0));
        v1 += v2;
        assert_approx_eq!(Vec3, v1, Vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn subtraction() {
        let mut v1 = Vec3(4.0, 5.0, 6.0);
        let v2 = Vec3(1.0, 2.0, 3.0);

        assert_approx_eq!(Vec3, v1 - v2, Vec3(3.0, 3.0, 3.0));
        v1 -= v2;
        assert_approx_eq!(Vec3, v1, Vec3(3.0, 3.0, 3.0));
        assert_approx_eq!(Vec3, -v1, Vec3(-3.0, -3.0, -3.0));
    }

    #[test]
    fn multiplication() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        let t = 2.0;

        assert_approx_eq!(Vec3, v1 * v2, Vec3(4.0, 10.0, 18.0));
        v1 *= v2;
        assert_approx_eq!(Vec3, v1, Vec3(4.0, 10.0, 18.0));
        assert_approx_eq!(Vec3, v1 * t, Vec3(8.0, 20.0, 36.0));
        v1 *= t;
        assert_approx_eq!(Vec3, v1, Vec3(8.0, 20.0, 36.0));
    }

    #[test]
    fn division() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let t = 2.0;

        assert_approx_eq!(Vec3, v1 / t, Vec3(0.5, 1.0, 1.5));
        v1 /= t;
        assert_approx_eq!(Vec3, v1, Vec3(0.5, 1.0, 1.5));
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

        assert_approx_eq!(Vec3, v1.cross(&v2), Vec3(-3.0, 6.0, -3.0));
    }
}
