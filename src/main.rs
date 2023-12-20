use std::sync::Arc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::sphere::SphereBuilder;
use crate::texture::checker::Checker;
use crate::texture::solid_color::SolidColor;
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
mod texture;
mod vec;
mod viewport;

fn main() {
    let test_size = 600;
    let image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());
    let root = BvhNode::new(vec![
        Arc::new(
            SphereBuilder::new(
                Vec3(0.0, -1000.3, 0.0),
                1000.0,
                Arc::new(Lambertian {
                    texture: Arc::new(Checker::new(
                        0.25,
                        Arc::new(SolidColor::black()),
                        Arc::new(SolidColor::white()),
                    )),
                }),
            )
            .build(),
        ),
        Arc::new(
            SphereBuilder::new(
                Vec3(0.0, 0.0, 0.5),
                0.3,
                Arc::new(Dielectric {
                    refraction_index: 1.458,
                }),
            )
            .build(),
        ),
        Arc::new(
            SphereBuilder::new(
                Vec3(-0.5, 0.0, -0.25),
                0.3,
                Arc::new(Lambertian {
                    texture: Arc::new(SolidColor(Color::new(0.9, 0.0, 0.0))),
                }),
            )
            .build(),
        ),
        Arc::new(
            SphereBuilder::new(
                Vec3(0.5, 0.0, -0.25),
                0.3,
                Arc::new(Metal {
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
        2.0,
        50.0,
        image,
    );

    let threads = 16;
    camera.samples = 4 * threads;
    camera
        .render_and_save(Arc::new(root), "output/result.png", threads)
        .unwrap();
}
