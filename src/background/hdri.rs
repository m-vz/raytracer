use std::f64::consts::PI;
use std::path::Path;

use crate::background::Background;
use crate::color::Color;
use crate::image::{Image, ImageError};
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Hdri {
    texture: Image,
    strength: f64,
    rotation: f64,
}

impl Hdri {
    pub fn load<P: AsRef<Path>>(path: P, strength: f64, rotation: f64) -> Result<Self, ImageError> {
        Ok(Self {
            texture: Image::load(path)?,
            strength,
            rotation: rotation.to_radians(),
        })
    }

    fn uv(&self, point: &Vec3) -> (f64, f64) {
        let theta = (-point.1).acos();
        let phi = (-point.2).atan2(point.0) + PI;

        ((phi + self.rotation) / (2.0 * PI), theta / PI)
    }
}

impl Background for Hdri {
    fn background(&self, ray: &Ray) -> Color {
        let (u, v) = self.uv(&ray.direction.normalized());

        self.strength * self.texture.get_pixel_by_uv(u, v)
    }
}
