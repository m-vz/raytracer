use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::vec::Vec3;

mod camera;
mod color;
mod image;
mod ray;
mod vec;
mod viewport;

fn main() {
    let test_size = 400;
    let image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());
    let mut camera = Camera::new(Vec3::zero(), Vec3::forward(), Vec3::up(), 1.0, 1.0, image);

    camera.render_to("output/test.ppm").unwrap()
}
