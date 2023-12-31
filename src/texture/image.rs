use std::path::Path;
use std::sync::Arc;

use crate::color::Color;
use crate::image::{Image, ImageError};
use crate::texture::Texture;
use crate::vec::Vec3;

pub struct ImageTexture {
    image: Arc<Image>,
}

impl ImageTexture {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        Ok(Self {
            image: Arc::new(Image::load(path)?),
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Vec3) -> Color {
        self.image.get_pixel_by_uv(u, v)
    }
}
