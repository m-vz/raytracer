use std::f64::consts::PI;

pub mod aabb;
pub mod interval;
pub mod perlin;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}
