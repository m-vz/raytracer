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
        let ray_object_space = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        }; // ray into object space
        let hit = self.object.hit(&ray_object_space, t_interval);

        if let Some(mut hit) = hit {
            hit.point += self.offset; // point into world space

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

pub struct RotationY {
    sinAngle: f64,
    cosAngle: f64,
    object: Arc<dyn Hit>,
    bounding_box: Aabb,
}

impl RotationY {
    pub fn new(object: Arc<dyn Hit>, angle: f64) -> Self {
        let angle = angle.to_radians();
    }
}

impl Hit for RotationY {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        let ray_object_space = Ray {
            origin: Vec3(
                self.cosAngle * ray.origin.0 - self.sinAngle * ray.origin.2,
                ray.origin.1,
                self.sinAngle * ray.origin.0 + self.cosAngle * ray.origin.2,
            ),
            direction: Vec3(
                self.cosAngle * ray.direction.0 - self.sinAngle * ray.direction.2,
                ray.direction.1,
                self.sinAngle * ray.direction.0 + self.cosAngle * ray.direction.2,
            ),
            time: ray.time,
        }; // ray into object space
        let hit = self.object.hit(&ray_object_space, t_interval);

        if let Some(mut hit) = hit {
            hit.point = Vec3(
                self.cosAngle * hit.point.0 + self.sinAngle * hit.point.2,
                hit.point.1,
                -self.sinAngle * hit.point.0 + self.cosAngle * hit.point.2,
            ); // intersection point into world space
            hit.normal = Vec3(
                self.cosAngle * hit.normal.0 + self.sinAngle * hit.normal.2,
                hit.normal.1,
                -self.sinAngle * hit.normal.0 + self.cosAngle * hit.normal.2,
            ); // normal into world space

            Some(hit)
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
