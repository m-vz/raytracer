use std::f64::consts::PI;
use std::ops::RangeBounds;
use std::sync::Arc;

use crate::hit::{Hit, HitResult};
use crate::material::Material;
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct SphereBuilder {
    pub center: Vec3,
    pub radius: f64,
    movement: Option<Vec3>,
    pub material: Arc<dyn Material>,
}

#[allow(dead_code)]
impl SphereBuilder {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            movement: None,
            material,
        }
    }

    pub fn with_movement(mut self, movement: Vec3) -> Self {
        self.movement = Some(movement);

        self
    }

    pub fn build(self) -> Sphere {
        let radius_vec = Vec3(self.radius, self.radius, self.radius);
        let bounding_box = if let Some(movement) = self.movement {
            let start = Aabb::with_extrema(self.center - radius_vec, self.center + radius_vec);
            let center_end = self.center + movement;
            let end = Aabb::with_extrema(center_end - radius_vec, center_end + radius_vec);

            start.combined(&end)
        } else {
            Aabb::with_extrema(self.center - radius_vec, self.center + radius_vec)
        };

        Sphere {
            center: self.center,
            radius: self.radius,
            movement: self.movement,
            material: self.material,
            bounding_box,
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub movement: Option<Vec3>,
    pub material: Arc<dyn Material>,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn center_at_time(&self, time: f64) -> Vec3 {
        if let Some(movement) = self.movement {
            self.center + time * movement
        } else {
            self.center
        }
    }

    fn uv(point: &Vec3) -> (f64, f64) {
        let theta = (-point.1).acos();
        let phi = (-point.2).atan2(point.0) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        let center = self.center_at_time(ray.time);
        let oc = ray.origin - center;
        let a = ray.direction.len_sq();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sq = discriminant.sqrt();
        let mut t = (-half_b - discriminant_sq) / a;
        if !t_interval.contains(&t) {
            t = (-half_b + discriminant_sq) / a;
            if !t_interval.contains(&t) {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - center) / self.radius;
        let uv = Self::uv(&outward_normal);
        Some(HitResult::new(
            ray,
            t,
            uv.0,
            uv.1,
            point,
            outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
