use std::ops::Range;

use crate::hit::{Hit, HitResult};
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Box<dyn Hit>>,
}

impl Hit for Scene {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitResult> {
        let mut closest_hit: Option<HitResult> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(
                &ray,
                t_range.start..if let Some(hit) = &closest_hit {
                    hit.t
                } else {
                    t_range.end
                },
            ) {
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
