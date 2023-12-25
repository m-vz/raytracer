use crate::color::Color;
use crate::math::perlin::PerlinNoise;
use crate::texture::Texture;
use crate::vec::Vec3;

pub struct Perlin {
    pub scale: f64,
    generator: PerlinNoise,
}

impl Perlin {
    pub fn new(scale: f64) -> Self {
        Self {
            generator: PerlinNoise::new(),
            scale,
        }
    }
}

impl Texture for Perlin {
    fn value(&self, _: f64, _: f64, point: Vec3) -> Color {
        0.5 * (1.0 + self.generator.noise(self.scale * point)) * Color::white()
    }
}
