use std::sync::Arc;

use crate::hit::{Hit, HitResult};
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Arc<dyn Hit>>,
    bounding_box: Aabb,
}

#[allow(unused)]
impl Scene {
    pub fn add(mut self, object: Arc<dyn Hit>) -> Self {
        self.bounding_box.combine(object.bounding_box());
        self.objects.push(object);

        self
    }
}

impl Hit for Scene {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        let mut closest_hit: Option<HitResult> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(
                ray,
                Interval(
                    t_interval.start()
                        ..closest_hit
                            .as_ref()
                            .map_or_else(|| t_interval.end(), |hit| hit.t),
                ),
            ) {
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
