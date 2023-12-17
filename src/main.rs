use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::scene::SceneBuilder;
use crate::sphere::SphereBuilder;
use crate::vec::Vec3;

mod camera;
mod color;
mod hit;
mod image;
mod material;
mod math;
mod ray;
mod scene;
mod sphere;
mod vec;
mod viewport;

fn main() {
    let test_size = 400;
    let image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());
    let scene = SceneBuilder::new()
        .add(
            SphereBuilder::new(
                Vec3(0.0, -1000.3, -1.0),
                1000.0,
                Rc::new(Lambertian {
                    albedo: Color::new(0.5, 0.5, 0.5),
                }),
            )
            .build(),
        )
        .add(
            SphereBuilder::new(
                Vec3(-0.5, 0.0, -1.0),
                0.3,
                Rc::new(Dielectric {
                    refraction_index: 1.5,
                }),
            )
            .build(),
        )
        .add(
            SphereBuilder::new(
                Vec3(0.0, 0.0, -1.0),
                0.3,
                Rc::new(Lambertian {
                    albedo: Color::new(0.0, 0.2, 0.8),
                }),
            )
            .build(),
        )
        .add(
            SphereBuilder::new(
                Vec3(0.5, 0.0, -1.0),
                0.3,
                Rc::new(Metal {
                    albedo: Color::new(0.8, 0.6, 0.2),
                    fuzz: 0.05,
                }),
            )
            .build(),
        )
        .build();
    let camera_position = Vec3(-1.0, 0.25, -0.25);
    let mut camera = Camera::new(
        camera_position,
        camera_position.look_at(&Vec3(0.0, 0.0, -1.3)),
        Vec3::up(),
        0.8,
        3.0,
        60.0,
        image,
    );

    camera.render(&scene, "output/test.ppm").unwrap()
}
