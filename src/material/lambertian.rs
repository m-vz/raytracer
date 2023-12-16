use float_cmp::{ApproxEq, F64Margin};

use crate::color::Color;
use crate::hit::HitResult;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Ray, Color)> {
        let mut direction = hit.normal + Vec3::random_unit_vector();

        if direction.approx_eq(Vec3::zero(), F64Margin::default()) {
            direction = hit.normal;
        }

        Some((
            Ray {
                origin: hit.point,
                direction,
                time: ray.time,
            },
            self.albedo,
        ))
    }
}
