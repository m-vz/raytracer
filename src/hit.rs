use std::ops::Range;

use crate::ray::Ray;
use crate::vec::Vec3;

pub struct HitResult {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitResult {
    pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        Self {
            t,
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            front_face,
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitResult>;
}
