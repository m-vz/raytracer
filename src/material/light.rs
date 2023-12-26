use std::sync::Arc;

use crate::color::Color;
use crate::hit::HitResult;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::vec::Vec3;

pub struct DiffuseLight {
    pub emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn colored(color: Color) -> Self {
        Self {
            emission: Arc::new(SolidColor(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitResult) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.emission.value(u, v, point)
    }
}
