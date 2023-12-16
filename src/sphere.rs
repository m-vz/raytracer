use crate::hit::{Hit, HitResult};
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_sq() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sq = discriminant.sqrt();
        let mut t = (-half_b - discriminant_sq) / a;
        if t <= t_min || t >= t_max {
            t = (-half_b + discriminant_sq) / a;
            if t <= t_min || t >= t_max {
                return None;
            }
        }

        let point = ray.at(t);
        Some(HitResult::new(
            ray,
            t,
            point,
            (point - self.center) / self.radius,
        ))
    }
}
