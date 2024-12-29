use crate::color::Color;
use crate::hit::HitResult;
use crate::material::Material;
use crate::ray::Ray;

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let direction_normalized = ray.direction.normalized();
        let cos_theta = -direction_normalized.dot(&hit.normal).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();

        if refraction_ratio * sin_theta > 1.0 || self.reflectance(cos_theta) > rand::random() {
            // reflect
            Some((
                Ray {
                    origin: hit.point,
                    direction: direction_normalized.reflect(&hit.normal),
                    time: ray.time,
                },
                Color::white(),
            ))
        } else {
            // refract
            Some((
                Ray {
                    origin: hit.point,
                    direction: direction_normalized.refract(&hit.normal, refraction_ratio),
                    time: ray.time,
                },
                Color::white(),
            ))
        }
    }
}

impl Dielectric {
    fn reflectance(&self, cos: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index).powi(2);

        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }
}
