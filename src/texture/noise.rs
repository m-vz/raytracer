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
            scale,
            generator: PerlinNoise::new(),
        }
    }
}

impl Texture for Perlin {
    fn value(&self, _: f64, _: f64, point: Vec3) -> Color {
        0.5 * (1.0 + self.generator.noise(self.scale * point)) * Color::white()
    }
}

pub struct TurbulentPerlin {
    pub scale: f64,
    pub turbulence: u32,
    generator: PerlinNoise,
}

impl TurbulentPerlin {
    pub fn new(scale: f64, turbulence: u32) -> Self {
        Self {
            scale,
            turbulence,
            generator: PerlinNoise::new(),
        }
    }
}

impl Texture for TurbulentPerlin {
    fn value(&self, _: f64, _: f64, point: Vec3) -> Color {
        self.generator
            .turbulence(self.scale * point, self.turbulence)
            * Color::white()
    }
}
