use std::sync::Arc;

use crate::color::Color;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::vec::Vec3;

pub struct Checker {
    inv_scale: f64,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_with_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self::new(scale, Arc::new(SolidColor(even)), Arc::new(SolidColor(odd)))
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        if ((p.0 * self.inv_scale).floor() as i32
            + (p.1 * self.inv_scale).floor() as i32
            + (p.2 * self.inv_scale).floor() as i32)
            % 2
            == 0
        {
            self.even.value(u, v, &p)
        } else {
            self.odd.value(u, v, &p)
        }
    }
}
