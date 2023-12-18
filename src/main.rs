use std::rc::Rc;

use rand::random;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hit::Hit;
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
    let test_size = 400;
    let image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());
    let sphere_radii = 0.3;
    let mut objects: Vec<Rc<dyn Hit>> = vec![
        Rc::new(
            SphereBuilder::new(
                Vec3(0.0, -1000.0 - sphere_radii, 0.0),
                1000.0,
                Rc::new(Lambertian {
                    albedo: Color::new(0.75, 0.75, 0.75),
                }),
            )
            .build(),
        ),
        Rc::new(
            SphereBuilder::new(
                Vec3(-0.8, 0.0, 0.0),
                sphere_radii,
                Rc::new(Dielectric {
                    refraction_index: 1.5,
                }),
            )
            .build(),
        ),
        Rc::new(
            SphereBuilder::new(
                Vec3(0.0, 0.0, 0.0),
                sphere_radii,
                Rc::new(Lambertian {
                    albedo: Color::new(0.9, 0.0, 0.0),
                }),
            )
            .build(),
        ),
        Rc::new(
            SphereBuilder::new(
                Vec3(0.8, 0.0, 0.0),
                sphere_radii,
                Rc::new(Metal {
                    albedo: Color::new(0.9, 0.6, 0.2),
                    fuzz: 0.05,
                }),
            )
            .build(),
        ),
    ];
    for _ in 0..100 {
        let radius = random::<f64>() * 0.05 + 0.001;
        let sphere: Rc<dyn Hit> = Rc::new(
            SphereBuilder::new(
                Vec3(
                    2.0 * random::<f64>() - 1.0,
                    radius - sphere_radii,
                    2.0 * random::<f64>() - 1.0,
                ),
                radius,
                Rc::new(Lambertian {
                    albedo: Color::new(0.0, random(), 0.0),
                }),
            )
            .build(),
        );
        objects.push(sphere);
    }
    let root = BvhNode::new(objects);
    let camera_position = Vec3(0.0, 0.25, 1.0);
    let mut camera = Camera::new(
        camera_position,
        camera_position.look_at(&Vec3::zero()),
        Vec3::up(),
        0.8,
        3.0,
        80.0,
        image,
    );

    camera.render(&root, "output/test.ppm").unwrap();
}
