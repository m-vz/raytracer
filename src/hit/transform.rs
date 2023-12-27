use std::sync::Arc;

use crate::hit::{Hit, HitResult};
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Translation {
    offset: Vec3,
    object: Arc<dyn Hit>,
    bounding_box: Aabb,
}

impl Translation {
    pub fn new(object: Arc<dyn Hit>, offset: Vec3) -> Self {
        let bounding_box = object.bounding_box().clone() + offset;

        Self {
            offset,
            object,
            bounding_box,
        }
    }
}

impl Hit for Translation {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        let translated_ray = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };
        let hit = self.object.hit(&translated_ray, t_interval);

        if let Some(mut hit) = hit {
            hit.point += self.offset;

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
