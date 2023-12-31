use crate::color::Color;
use crate::ray::Ray;

pub mod background_color;
pub mod hdri;

pub trait Background: Send + Sync {
    fn background(&self, ray: &Ray) -> Color;
}
