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
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }

    fn normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.center).normalized()
    }
}
