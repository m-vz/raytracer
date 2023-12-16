use crate::color::Color;
use crate::hit::HitResult;
use crate::material::Material;
use crate::ray::Ray;

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Ray, Color)> {
        Some((
            Ray {
                origin: hit.point,
                direction: ray.direction.normalized().reflect(&hit.normal),
            },
            self.albedo,
        ))
    }
}
