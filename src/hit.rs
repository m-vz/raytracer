use std::sync::Arc;

use crate::material::Material;
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;

pub mod r#box;
pub mod bvh;
pub mod quad;
pub mod scene;
pub mod sphere;

pub struct HitResult {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitResult {
    pub fn new(
        ray: &Ray,
        t: f64,
        u: f64,
        v: f64,
        point: Vec3,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        Self {
            t,
            u,
            v,
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material,
            front_face,
        }
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult>;

    fn bounding_box(&self) -> &Aabb;
}
