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
    sin_angle: f64,
    cos_angle: f64,
    object: Arc<dyn Hit>,
    bounding_box: Aabb,
}

impl RotationY {
    pub fn new(object: Arc<dyn Hit>, angle: f64) -> Self {
        let angle = angle.to_radians();
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();
        let bounding_box = object.bounding_box();

        let mut min = Vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    let mut test = Vec3(
                        x as f64 * bounding_box.0.end() + (1 - x) as f64 * bounding_box.0.start(),
                        y as f64 * bounding_box.1.end() + (1 - y) as f64 * bounding_box.1.start(),
                        z as f64 * bounding_box.2.end() + (1 - z) as f64 * bounding_box.2.start(),
                    );

                    test.0 = cos_angle * test.0 + sin_angle * test.2;
                    test.2 = -sin_angle * test.0 + cos_angle * test.2;

                    for i in 0..3 {
                        min.set_axis(i, min.axis(i).min(test.axis(i)));
                        max.set_axis(i, max.axis(i).max(test.axis(i)));
                    }
                }
            }
        }

        Self {
            sin_angle,
            cos_angle,
            object,
            bounding_box: Aabb::with_extrema(min, max),
        }
    }
}

impl Hit for RotationY {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        let ray_object_space = Ray {
            origin: Vec3(
                self.cos_angle * ray.origin.0 - self.sin_angle * ray.origin.2,
                ray.origin.1,
                self.sin_angle * ray.origin.0 + self.cos_angle * ray.origin.2,
            ),
            direction: Vec3(
                self.cos_angle * ray.direction.0 - self.sin_angle * ray.direction.2,
                ray.direction.1,
                self.sin_angle * ray.direction.0 + self.cos_angle * ray.direction.2,
            ),
            time: ray.time,
        }; // ray into object space
        let hit = self.object.hit(&ray_object_space, t_interval);

        if let Some(mut hit) = hit {
            hit.point = Vec3(
                self.cos_angle * hit.point.0 + self.sin_angle * hit.point.2,
                hit.point.1,
                -self.sin_angle * hit.point.0 + self.cos_angle * hit.point.2,
            ); // intersection point into world space
            hit.normal = Vec3(
                self.cos_angle * hit.normal.0 + self.sin_angle * hit.normal.2,
                hit.normal.1,
                -self.sin_angle * hit.normal.0 + self.cos_angle * hit.normal.2,
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