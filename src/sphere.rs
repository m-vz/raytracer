use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_sq() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }

    fn normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.center).normalized()
    }
}
