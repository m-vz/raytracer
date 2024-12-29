use std::ops::RangeBounds;
use std::sync::Arc;

use float_cmp::{ApproxEq, F64Margin};

use crate::hit::{Hit, HitResult};
use crate::material::Material;
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Quad {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub material: Arc<dyn Material>,
    bounding_box: Aabb,
    normal: Vec3,
    d: f64,  // plane coefficient
    w: Vec3, // basis frame helper vector
}

impl Quad {
    pub fn new(origin: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let mut normal = u.cross(&v);
        let w = normal / normal.dot(&normal);
        normal.normalize();

        Self {
            origin,
            u,
            v,
            material,
            bounding_box: Aabb::with_extrema(origin, origin + u + v).padded(0.0001),
            normal,
            d: normal.dot(&origin),
            w,
        }
    }
}

impl Hit for Quad {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        let denominator = self.normal.dot(&ray.direction);
        if denominator.abs().approx_eq(0.0, F64Margin::default()) {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denominator;
        if !t_interval.contains(&t) {
            return None;
        }

        let point = ray.at(t);
        let local_point = point - self.origin;
        let u = self.w.dot(&local_point.cross(&self.v));
        let v = self.w.dot(&self.u.cross(&local_point));
        if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
            return None;
        }

        Some(HitResult::new(
            ray,
            t,
            u,
            v,
            point,
            self.normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
