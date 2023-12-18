use std::rc::Rc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::sphere::SphereBuilder;
use crate::vec::Vec3;

mod bvh;
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
    let test_size = 800;
    let image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());
    let root = BvhNode::new(vec![
        Rc::new(
            SphereBuilder::new(
                Vec3(0.0, -1000.3, 0.0),
                1000.0,
                Rc::new(Lambertian {
                    albedo: Color::new(0.75, 0.75, 0.75),
                }),
            )
            .build(),
        ),
        Rc::new(
            SphereBuilder::new(
                Vec3(0.0, 0.0, 0.5),
                0.3,
                Rc::new(Dielectric {
                    refraction_index: 1.5,
                }),
            )
            .build(),
        ),
        Rc::new(
            SphereBuilder::new(
                Vec3(-0.5, 0.0, -0.25),
                0.3,
                Rc::new(Lambertian {
                    albedo: Color::new(0.9, 0.0, 0.0),
                }),
            )
            .build(),
        ),
        Rc::new(
            SphereBuilder::new(
                Vec3(0.5, 0.0, -0.25),
                0.3,
                Rc::new(Metal {
                    albedo: Color::new(0.9, 0.6, 0.2),
                    fuzz: 0.05,
                }),
            )
            .build(),
        ),
    ]);
    let camera_position = Vec3(0.0, 0.25, 2.0);
    let mut camera = Camera::new(
        camera_position,
        camera_position.look_at(&Vec3(0.0, 0.0, 0.5)),
        Vec3::up(),
        0.8,
        3.0,
        50.0,
        image,
    );
    camera.samples = 10;

    camera.render(&root, "output/test.ppm").unwrap();
}
