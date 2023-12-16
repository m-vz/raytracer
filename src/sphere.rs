use std::ops::Range;
use std::rc::Rc;

use crate::hit::{Hit, HitResult};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sq = discriminant.sqrt();
        let mut t = (-half_b - discriminant_sq) / a;
        if !t_range.contains(&t) {
            t = (-half_b + discriminant_sq) / a;
            if !t_range.contains(&t) {
                return None;
            }
        }

        let point = ray.at(t);
        Some(HitResult::new(
            ray,
            t,
            point,
            (point - self.center) / self.radius,
            Rc::clone(&self.material),
        ))
    }
}
