use crate::hit::{Hit, HitResult};
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Box<dyn Hit>>,
}

impl Hit for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut closest_hit: Option<HitResult> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(
                &ray,
                t_min,
                if let Some(hit) = &closest_hit {
                    hit.t
                } else {
                    t_max
                },
            ) {
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
