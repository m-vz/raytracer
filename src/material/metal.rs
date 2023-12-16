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
            let scattered = ray.direction.normalized().reflect(&hit.normal)
                + self.fuzz * Vec3::random_unit_vector();

            if scattered.dot(&hit.normal) > 0.0 {
                return Some((
                    Ray {
                        origin: hit.point,
                        direction: scattered,
                    },
                    self.albedo,
                ));
            }
        }
    }
}
