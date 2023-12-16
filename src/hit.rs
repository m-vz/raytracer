use crate::ray::Ray;
use crate::vec::Vec3;

pub struct HitResult {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}
