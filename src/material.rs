use crate::color::Color;
use crate::hit::HitResult;
use crate::ray::Ray;
use crate::vec::Vec3;

pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;

pub trait Material: Send + Sync {
    /// Scatter an incoming ray and produce an outgoing ray and attenuation
    ///
    /// # Arguments
    ///
    /// * `ray`: The incoming ray.
    /// * `hit`: Where the ray hit the object.
    ///
    /// Returns a tuple with the new outgoing ray and its attenuation or `None` if the ray was
    /// absorbed.
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> Option<(Ray, Color)>;

    fn emitted(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        Color::black()
    }
}
