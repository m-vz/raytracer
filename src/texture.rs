use crate::color::Color;
use crate::vec::Vec3;

pub mod checker;
pub mod image;
pub mod noise;
pub mod solid_color;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color;
}
