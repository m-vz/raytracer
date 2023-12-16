use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::material::lambertian::Lambertian;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vec::Vec3;

mod camera;
mod color;
mod hit;
mod image;
mod material;
mod ray;
mod scene;
mod sphere;
mod vec;
mod viewport;

fn main() {
    let test_size = 800;
    let image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());
    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                center: Vec3(0.0, -1000.25, -1.0),
                radius: 1000.0,
                material: Rc::new(Lambertian {
                    albedo: Color::new(0.5, 0.5, 0.5),
                }),
            }),
            Box::new(Sphere {
                center: Vec3(0.0, 0.0, -1.0),
                radius: 0.25,
                material: Rc::new(Lambertian {
                    albedo: Color::new(1.0, 0.0, 0.0),
                }),
            }),
        ],
    };
    let mut camera = Camera::new(Vec3::zero(), Vec3::forward(), Vec3::up(), 1.0, 1.0, image);

    camera.render(&scene, "output/test.ppm").unwrap()
}
