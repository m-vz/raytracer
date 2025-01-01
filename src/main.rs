#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]

use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use camera::{CameraBuilder, CameraError};
use ui::Preview;

use crate::background::background_color::BackgroundColor;
use crate::background::hdri::Hdri;
use crate::camera::Camera;
use crate::color::Color;
use crate::hit::bvh::Node;
use crate::hit::quad::Quad;
use crate::hit::r#box::Box;
use crate::hit::sphere::SphereBuilder;
use crate::hit::transform::{RotationY, Translation};
use crate::hit::Hit;
use crate::image::Image;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::light::DiffuseLight;
use crate::material::metal::Metal;
use crate::texture::checker::Checker;
use crate::texture::image::ImageTexture;
use crate::texture::noise::{Perlin, TurbulentPerlin};
use crate::texture::solid_color::SolidColor;
use crate::vec::Vec3;

mod background;
mod camera;
mod color;
mod hit;
mod image;
mod material;
mod math;
mod ray;
mod texture;
mod ui;
mod vec;
mod viewport;

fn main() {
    let width = 512;
    let num_threads = 8;
    let image = Image::with_aspect_ratio(width, 1.0, Color::black());
    let (camera, root) = mis(&image);
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let render_thread = thread::spawn(move || {
        println!("starting render...");

        let t = Instant::now();
        let threads = start_render(&camera, &root, &image, num_threads);
        combine_results(threads, num_threads)
            .expect("could not combine images")
            .write_png("output/result.png", true)
            .expect("could not write image");

        println!("done in {}ms", t.elapsed().as_millis());

        shutdown_tx.send(()).expect("could not shut down ui");
    });
    Preview::new(shutdown_rx).run();
    render_thread.join().expect("could not join render thread");
}

fn start_render(
    camera: &Camera,
    root: &Arc<dyn Hit>,
    target: &Image,
    num_threads: u32,
) -> Vec<JoinHandle<Image>> {
    let samples_per_thread = f64::from(camera.samples) / f64::from(num_threads);
    let mut threads = Vec::with_capacity(num_threads as usize);

    for i in 0..num_threads {
        let thread_camera = camera.clone();
        let thread_target = target.clone();
        let thread_root = root.clone();

        threads.push(thread::spawn(move || {
            thread_camera.render(&thread_root, samples_per_thread, thread_target, i == 0)
        }));
    }

    threads
}

fn combine_results(
    threads: Vec<JoinHandle<Image>>,
    num_threads: u32,
) -> Result<Image, CameraError> {
    let mut thread_targets = Vec::with_capacity(num_threads as usize);
    threads
        .into_iter()
        .for_each(|t| thread_targets.push(t.join().unwrap()));

    println!("combining images...");
    Image::average(&thread_targets).map_err(CameraError::Averaging)
}

#[allow(dead_code)]
fn mis(image: &Image) -> (Camera, Arc<dyn Hit>) {
    let incline = 0.13;
    let width = 1.4;
    (
        CameraBuilder::new(8.0, 0.0, 40.0)
            .with_position(Vec3(0.0, 3.0, 5.0))
            .with_forward(Vec3(0.0, -0.5, -2.0))
            .with_samples(100)
            .with_max_bounces(2) // don't show the wall reflection in the mirrors
            .build(image),
        Arc::new(Node::new(vec![
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, 3.0, -1006.0),
                    1000.0,
                    Arc::new(Lambertian::colored(Color::white())),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, -1000.01, 0.0),
                    1000.0,
                    Arc::new(Lambertian::colored(Color::white())),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(-2.25, 3.0, -5.0),
                    0.02,
                    Arc::new(DiffuseLight::colored(Color::white())),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(-0.75, 3.0, -5.0),
                    0.1,
                    Arc::new(DiffuseLight::colored(Color::red())),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.75, 3.0, -5.0),
                    0.25,
                    Arc::new(DiffuseLight::colored(Color::green())),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(2.25, 3.0, -5.0),
                    0.5,
                    Arc::new(DiffuseLight::colored(Color::blue())),
                )
                .build(),
            ),
            Arc::new(Quad::new(
                Vec3(-2.5, 0.0, 0.5 * width),
                5.0 * Vec3::right(),
                width * Vec3(0.0, 0.0, -1.0),
                Arc::new(Metal {
                    albedo: Color::white(),
                    fuzz: 0.08,
                }),
            )),
            Arc::new(Quad::new(
                Vec3(-2.5, 0.0, -0.5 * width),
                5.0 * Vec3::right(),
                width * Vec3(0.0, incline, -1.0),
                Arc::new(Metal {
                    albedo: Color::white(),
                    fuzz: 0.05,
                }),
            )),
            Arc::new(Quad::new(
                Vec3(-2.5, incline, -1.5 * width),
                5.0 * Vec3::right(),
                width * Vec3(0.0, 2.2 * incline, -1.0),
                Arc::new(Metal {
                    albedo: Color::white(),
                    fuzz: 0.01,
                }),
            )),
            Arc::new(Quad::new(
                Vec3(-2.5, 3.2 * incline, -2.5 * width),
                5.0 * Vec3::right(),
                width * Vec3(0.0, 4.2 * incline, -1.0),
                Arc::new(Metal {
                    albedo: Color::white(),
                    fuzz: 0.0,
                }),
            )),
        ])),
    )
}

#[allow(dead_code)]
fn hdri(image: &Image) -> (Camera, Arc<dyn Hit>) {
    let white = Arc::new(Lambertian::colored(Color::white()));
    let a = 5.55;

    (
        CameraBuilder::new(8.0, 0.0, 40.0)
            .with_position(Vec3(2.78, 2.78, -8.0))
            .with_forward(-Vec3::forward())
            .with_background(Hdri::load(
                "resources/hdri/drakensberg_solitary_mountain_puresky_4k.hdr",
                1.0,
                0.0,
            ))
            .build(image),
        Arc::new(Node::new(vec![
            // left
            Arc::new(Quad::new(
                a * Vec3::right(),
                a * -Vec3::forward(),
                a * Vec3::up(),
                Arc::new(Lambertian::colored(Color::new(0.12, 0.45, 0.15))),
            )),
            // right
            Arc::new(Quad::new(
                Vec3::zero(),
                a * -Vec3::forward(),
                a * Vec3::up(),
                Arc::new(Lambertian::colored(Color::new(0.65, 0.05, 0.05))),
            )),
            // bottom
            Arc::new(Quad::new(
                Vec3::zero(),
                a * Vec3::right(),
                a * -Vec3::forward(),
                white.clone(),
            )),
            // top
            Arc::new(Quad::new(
                a * Vec3::up(),
                a * Vec3::right(),
                a * -Vec3::forward(),
                white.clone(),
            )),
            // tall box
            Arc::new(Translation::new(
                Arc::new(RotationY::new(
                    Arc::new(Box::new(Vec3::zero(), Vec3(1.65, 3.3, 1.65), white.clone())),
                    15.0,
                )),
                Vec3(2.65, 0.0, 2.95),
            )),
            // small box
            Arc::new(Translation::new(
                Arc::new(RotationY::new(
                    Arc::new(Box::new(Vec3::zero(), Vec3(1.65, 1.65, 1.65), white)),
                    -18.0,
                )),
                Vec3(1.3, 0.0, 0.65),
            )),
        ])),
    )
}

#[allow(dead_code)]
fn cornell_box(image: &Image) -> (Camera, Arc<dyn Hit>) {
    let white = Arc::new(Lambertian::colored(Color::white()));
    let a = 555.0;

    (
        CameraBuilder::new(800.0, 0.0, 40.0)
            .with_position(Vec3(278.0, 278.0, -800.0))
            .with_forward(-Vec3::forward())
            .build(image),
        Arc::new(Node::new(vec![
            Arc::new(Quad::new(
                a * Vec3::right(),
                a * Vec3::up(),
                a * -Vec3::forward(),
                Arc::new(Lambertian::colored(Color::new(0.12, 0.45, 0.15))),
            )),
            Arc::new(Quad::new(
                Vec3::zero(),
                a * Vec3::up(),
                a * -Vec3::forward(),
                Arc::new(Lambertian::colored(Color::new(0.65, 0.05, 0.05))),
            )),
            Arc::new(Quad::new(
                Vec3(343.0, 554.0, 332.0),
                Vec3(-130.0, 0.0, 0.0),
                Vec3(0.0, 0.0, -105.0),
                Arc::new(DiffuseLight::colored(Color::new(15.0, 15.0, 15.0))),
            )),
            Arc::new(Quad::new(
                Vec3::zero(),
                a * Vec3::right(),
                a * -Vec3::forward(),
                white.clone(),
            )),
            Arc::new(Quad::new(
                a * Vec3::unit(),
                a * -Vec3::right(),
                a * Vec3::forward(),
                white.clone(),
            )),
            Arc::new(Quad::new(
                a * -Vec3::forward(),
                a * Vec3::right(),
                a * Vec3::up(),
                white.clone(),
            )),
            Arc::new(Translation::new(
                Arc::new(RotationY::new(
                    Arc::new(Box::new(
                        Vec3::zero(),
                        Vec3(165.0, 330.0, 165.0),
                        white.clone(),
                    )),
                    15.0,
                )),
                Vec3(265.0, 0.0, 295.0),
            )),
            Arc::new(Translation::new(
                Arc::new(RotationY::new(
                    Arc::new(Box::new(Vec3::zero(), Vec3(165.0, 165.0, 165.0), white)),
                    -18.0,
                )),
                Vec3(130.0, 0.0, 65.0),
            )),
        ])),
    )
}

#[allow(dead_code)]
fn light(image: &Image) -> (Camera, Arc<dyn Hit>) {
    (
        CameraBuilder::new(8.0, 2.0, 60.0)
            .with_position(Vec3(0.0, 3.2, 8.0))
            .build(image),
        Arc::new(Node::new(vec![
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, -1000.0, 0.0),
                    1000.0,
                    Arc::new(Lambertian::colored(Color::white())),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, 2.0, 0.0),
                    2.0,
                    Arc::new(Metal {
                        albedo: Color::white(),
                        fuzz: 0.4,
                    }),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(-2.0, 5.6, 0.0),
                    1.0,
                    Arc::new(DiffuseLight::colored(Color::new(0.1, 4.0, 0.1))),
                )
                .build(),
            ),
            Arc::new(Quad::new(
                Vec3(1.0, 0.0, -3.0),
                4.0 * Vec3::right(),
                4.0 * Vec3::up(),
                Arc::new(DiffuseLight::colored(Color::new(4.0, 0.1, 0.1))),
            )),
        ])),
    )
}

#[allow(dead_code)]
fn quads(image: &Image) -> (Camera, Arc<dyn Hit>) {
    (
        CameraBuilder::new(9.0, 0.0, 80.0)
            .with_position(Vec3(0.0, 0.0, 9.0))
            .with_background(BackgroundColor::new(0.7, 0.8, 1.0))
            .look_at(Vec3::zero())
            .build(image),
        Arc::new(Node::new(vec![
            Arc::new(Quad::new(
                Vec3(-3.0, -2.0, 5.0),
                4.0 * Vec3::forward(),
                4.0 * Vec3::up(),
                Arc::new(Lambertian {
                    texture: Arc::new(SolidColor::new(1.0, 0.2, 0.2)),
                }),
            )),
            Arc::new(Quad::new(
                Vec3(-2.0, -2.0, 0.0),
                4.0 * Vec3::right(),
                4.0 * Vec3::up(),
                Arc::new(Lambertian {
                    texture: Arc::new(SolidColor::new(0.2, 1.0, 0.2)),
                }),
            )),
            Arc::new(Quad::new(
                Vec3(3.0, -2.0, 1.0),
                -4.0 * Vec3::forward(),
                4.0 * Vec3::up(),
                Arc::new(Lambertian {
                    texture: Arc::new(SolidColor::new(0.2, 0.2, 1.0)),
                }),
            )),
            Arc::new(Quad::new(
                Vec3(-2.0, 3.0, 1.0),
                4.0 * Vec3::right(),
                -4.0 * Vec3::forward(),
                Arc::new(Lambertian {
                    texture: Arc::new(SolidColor::new(1.0, 0.5, 0.0)),
                }),
            )),
            Arc::new(Quad::new(
                Vec3(-2.0, -3.0, 5.0),
                4.0 * Vec3::right(),
                4.0 * Vec3::forward(),
                Arc::new(Lambertian {
                    texture: Arc::new(SolidColor::new(0.2, 0.8, 0.8)),
                }),
            )),
        ])),
    )
}

#[allow(dead_code)]
fn noise(image: &Image) -> (Camera, Arc<dyn Hit>) {
    (
        CameraBuilder::new(3.0, 0.0, 20.0)
            .with_position(Vec3(13.0, 2.0, 3.0))
            .with_background(BackgroundColor::new(0.7, 0.8, 1.0))
            .look_at(Vec3::zero())
            .build(image),
        Arc::new(Node::new(vec![
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, -1000.0, 0.0),
                    1000.0,
                    Arc::new(Lambertian {
                        texture: Arc::new(TurbulentPerlin::new(1.0, 10)),
                    }),
                )
                .build(),
            ),
            Arc::new(
                SphereBuilder::new(
                    Vec3(0.0, 2.0, 0.0),
                    2.0,
                    Arc::new(Lambertian {
                        texture: Arc::new(Perlin::new(8.0)),
                    }),
                )
                .build(),
            ),
        ])),
    )
}

#[allow(dead_code)]
fn earth(image: &Image) -> (Camera, Arc<dyn Hit>) {
    let material = Arc::new(Lambertian {
        texture: Arc::new(ImageTexture::load("resources/earth.png")),
    });
    let mut spheres: Vec<Arc<dyn Hit>> = Vec::new();
    for z in -2..=2 {
        for y in 0..=1 {
            for x in -2..=2 {
                spheres.push(Arc::new(
                    SphereBuilder::new(
                        Vec3(f64::from(x) * 0.5, f64::from(y) * 0.5, f64::from(z)),
                        0.2,
                        if z == 2 && y == 0 && x == 0 {
                            Arc::new(Dielectric {
                                refraction_index: 1.458,
                            })
                        } else {
                            material.clone()
                        },
                    )
                    .build(),
                ));
            }
        }
    }
    spheres.push(Arc::new(
        SphereBuilder::new(Vec3(0.0, -1000.2, 0.0), 1000.0, material).build(),
    ));

    (
        CameraBuilder::new(2.0, 1.0, 20.0)
            .with_position(Vec3(0.0, 0.0, 4.0))
            .with_background(BackgroundColor::new(0.7, 0.8, 1.0))
            .look_at(Vec3::zero())
            .build(image),
        Arc::new(Node::new(spheres)),
    )
}

#[allow(dead_code)]
fn checker_balls(image: &Image) -> (Camera, Arc<dyn Hit>) {
    let camera_position = Vec3(0.0, 0.25, 1.0);

    (
        CameraBuilder::new(camera_position.len(), 2.0, 40.0)
            .with_position(camera_position)
            .with_background(BackgroundColor::new(0.7, 0.8, 1.0))
            .look_at(Vec3::zero())
            .build(image),
        Arc::new(Node::new(vec![
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
                    Arc::new(Lambertian::colored(Color::new(0.9, 0.0, 0.0))),
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
