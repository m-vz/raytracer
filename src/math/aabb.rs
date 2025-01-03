use std::ops::Add;

use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Default, Clone)]
pub struct Aabb(pub Interval, pub Interval, pub Interval);

#[allow(dead_code)]
impl Aabb {
    pub fn with_extrema(a: Vec3, b: Vec3) -> Self {
        Self(
            Interval(a.0.min(b.0)..a.0.max(b.0)),
            Interval(a.1.min(b.1)..a.1.max(b.1)),
            Interval(a.2.min(b.2)..a.2.max(b.2)),
        )
    }

    pub fn pad(&mut self, delta: f64) {
        self.0.pad(delta);
        self.1.pad(delta);
        self.2.pad(delta);
    }

    pub fn padded(&self, delta: f64) -> Self {
        Self(
            self.0.padded(delta),
            self.1.padded(delta),
            self.2.padded(delta),
        )
    }

    pub fn expand(&mut self, delta: f64) {
        self.0.expand(delta);
        self.1.expand(delta);
        self.2.expand(delta);
    }

    pub fn expanded(&self, delta: f64) -> Self {
        Self(
            self.0.expanded(delta),
            self.1.expanded(delta),
            self.2.expanded(delta),
        )
    }

    pub fn combine(&mut self, rhs: &Self) {
        self.0.combine(&rhs.0);
        self.1.combine(&rhs.1);
        self.2.combine(&rhs.2);
    }

    pub fn combined(&self, rhs: &Self) -> Self {
        Self(
            self.0.combined(&rhs.0),
            self.1.combined(&rhs.1),
            self.2.combined(&rhs.2),
        )
    }

    pub fn axis(&self, n: u32) -> &Interval {
        match n {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> bool {
        for i in 0..3 {
            let direction_inv = 1.0 / ray.direction.axis(i);
            let origin = ray.origin.axis(i);
            let mut t0 = (self.axis(i).start() - origin) * direction_inv;
            let mut t1 = (self.axis(i).end() - origin) * direction_inv;

            if direction_inv < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_interval.start() {
                t_interval.set_start(t0);
            }
            if t1 < t_interval.end() {
                t_interval.set_end(t1);
            }
            if t_interval.end() <= t_interval.start() {
                return false;
            }
        }

        true
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: Aabb) -> Self::Output {
        rhs + self
    }
}
