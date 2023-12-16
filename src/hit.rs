use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Hit {
    fn hit(&self, ray: &Ray) -> f64;

    fn normal(&self, point: &Vec3) -> Vec3;
}
