use std::sync::Arc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hit::Hit;
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
    let width = 600;
    let image = Image::with_aspect_ratio(width, 16.0 / 9.0, Color::black());

    let (mut camera, root) = scene_b(image);

    let threads = 16;
    camera.samples = 4 * threads;
    camera
        .render_and_save(root, "output/result.png", threads)
        .unwrap();
}

#[allow(dead_code)]
fn scene_b(image: Image) -> (Camera, Arc<dyn Hit>) {
    let material = Arc::new(Lambertian {
        texture: Arc::new(Checker::new_with_colors(
            0.4,
            Color::new(0.2, 0.3, 0.1),
            0.9 * Color::white(),
        )),
    });

    (
        Camera::look_at(
            Vec3(13.0, 2.0, 3.0),
            Vec3::zero(),
            Vec3::up(),
            3.0,
            0.0,
            20.0,
            image,
        ),
        Arc::new(BvhNode::new(vec![
            Arc::new(SphereBuilder::new(Vec3(0.0, -10.0, 0.0), 10.0, material.clone()).build()),
            Arc::new(SphereBuilder::new(Vec3(0.0, 10.0, 0.0), 10.0, material).build()),
        ])),
    )
}

#[allow(dead_code)]
fn scene_a(image: Image) -> (Camera, Arc<dyn Hit>) {
    let camera_position = Vec3(0.0, 0.25, 1.0);

    (
        Camera::look_at(
            camera_position,
            Vec3::zero(),
            Vec3::up(),
            camera_position.len(),
            2.0,
            40.0,
            image,
        ),
        Arc::new(BvhNode::new(vec![
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, -1000.3, 0.0),
                    1000.0,
                    Arc::new(Lambertian {
                        texture: Arc::new(Checker::new_with_colors(
                            0.25,
                            Color::black(),
                            Color::white(),
                        )),
                    }),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3::zero(),
                    0.3,
                    Arc::new(Dielectric {
                        refraction_index: 1.458,
                    }),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(-0.5, 0.0, -0.75),
                    0.3,
                    Arc::new(Lambertian {
                        texture: Arc::new(SolidColor(Color::new(0.9, 0.0, 0.0))),
                    }),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.5, 0.0, -0.75),
                    0.3,
                    Arc::new(Metal {
                        albedo: Color::new(0.9, 0.6, 0.2),
                        fuzz: 0.05,
                    }),
                )
                .build(),
            ),
        ])),
    )
}
