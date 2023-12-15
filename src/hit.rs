use crate::ray::Ray;

pub trait Hit {
    fn hit(&self, ray: &Ray) -> bool;
}
