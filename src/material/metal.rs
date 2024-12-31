use crate::color::Color;
use crate::hit::HitResult;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Ray, Color)> {
        loop {
            let mut scattered = ray.direction.normalized().reflect(&hit.normal);
            if self.fuzz > 0.0 {
                scattered += self.fuzz * Vec3::random_unit_vector();
                scattered.normalize();
            }

            if scattered.dot(&hit.normal) > 0.0 {
                return Some((
                    Ray {
                        origin: hit.point,
                        direction: scattered,
                        time: ray.time,
                    },
                    self.albedo,
                ));
            }
        }
    }
}
