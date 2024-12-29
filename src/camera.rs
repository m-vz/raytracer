use std::io;
use std::path::Path;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Instant;

use crate::background::background_color::BackgroundColor;
use crate::background::Background;
use crate::color::Color;
use crate::hit::Hit;
use crate::image::{Image, ImageError};
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::viewport::Viewport;

const BIAS: f64 = 0.001;

#[derive(Debug)]
pub enum CameraError {
    IOError(io::Error),
    Averaging(ImageError),
}

impl From<io::Error> for CameraError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

#[derive(Clone)]
pub struct CameraBuilder {
    position: Vec3,
    forward: Vec3,
    up: Vec3,
    focus_distance: f64,
    defocus_angle: f64,
    fov: f64,
    samples: u32,
    max_bounces: u32,
    background: Arc<dyn Background>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            position: Vec3::zero(),
            forward: Vec3::forward(),
            up: Vec3::up(),
            focus_distance: 1.0,
            defocus_angle: 0.0,
            fov: 80.0,
            samples: 9,
            max_bounces: 50,
            background: Arc::new(BackgroundColor::default()),
        }
    }
}

#[allow(dead_code)]
impl CameraBuilder {
    pub fn new(focus_distance: f64, defocus_angle: f64, fov: f64) -> Self {
        Self {
            focus_distance,
            defocus_angle,
            fov,
            ..Default::default()
        }
    }

    pub const fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub const fn with_forward(mut self, forward: Vec3) -> Self {
        self.forward = forward;
        self
    }

    pub const fn with_up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    pub const fn with_samples(mut self, samples: u32) -> Self {
        self.samples = samples;
        self
    }

    pub const fn with_max_bounces(mut self, max_bounces: u32) -> Self {
        self.max_bounces = max_bounces;
        self
    }

    pub fn with_background(mut self, background: impl Background + 'static) -> Self {
        self.background = Arc::new(background);
        self
    }

    pub fn look_at(mut self, look_at: Vec3) -> Self {
        self.forward = self.position.look_at(&look_at);
        self
    }

    pub fn build(mut self, target: Image) -> Camera {
        self.forward.normalize();
        let right = self.forward.cross(&self.up.normalized());
        let up = right.cross(&self.forward);

        let h = (self.fov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;

        let viewport = Viewport::with_center(
            self.position + self.focus_distance * self.forward,
            (viewport_height * target.aspect(), viewport_height),
            target.resolution(),
            right,
            -up,
        );

        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();

        Camera {
            position: self.position,
            viewport,
            defocus_disk: (right * defocus_radius, up * defocus_radius),
            target,
            samples: 9,
            max_bounces: 50,
            background: self.background,
        }
    }
}

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3,
    viewport: Viewport,
    defocus_disk: (Vec3, Vec3),
    target: Image,
    pub samples: u32,
    pub max_bounces: u32,
    pub background: Arc<dyn Background>,
}

impl Camera {
    pub fn render_and_save<P: AsRef<Path>>(
        &mut self,
        root: &Arc<dyn Hit>,
        path: P,
        num_threads: u32,
    ) -> Result<(), CameraError> {
        println!("starting render...");
        let t = Instant::now();
        let samples_per_thread = f64::from(self.samples) / f64::from(num_threads);

        if num_threads > 1 {
            let mut threads = Vec::with_capacity(num_threads as usize);
            for i in 0..num_threads {
                let mut thread_camera = self.clone();
                let thread_root = root.clone();

                threads.push(spawn(move || {
                    thread_camera.render(&thread_root, samples_per_thread, i == 0);
                    thread_camera.target
                }));
            }

            let mut images = Vec::with_capacity(num_threads as usize);
            threads
                .into_iter()
                .for_each(|t| images.push(t.join().unwrap()));

            println!("combining images...");
            match Image::average(&images) {
                Ok(average) => self.target = average,
                Err(error) => return Err(CameraError::Averaging(error)),
            }
        } else {
            self.render(root, samples_per_thread, true);
        }

        println!("writing file...");
        if let Err(ImageError::IOError(error)) = self.target.write_png(path, true) {
            return Err(error.into());
        }

        println!("done in {}ms", t.elapsed().as_millis());
        Ok(())
    }

    fn render(&mut self, root: &Arc<dyn Hit>, samples: f64, log: bool) {
        let samples_sqrt = samples.sqrt();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let dimension_indices = 0..samples_sqrt as u32;
        let subpixel_scale = 1.0 / samples_sqrt;

        for y in 0..self.target.height() {
            for x in 0..self.target.width() {
                let mut color = Color::black();

                for sample_y in dimension_indices.clone() {
                    for sample_x in dimension_indices.clone() {
                        let sample =
                            self.viewport
                                .pixel_sample(x, y, sample_x, sample_y, subpixel_scale);
                        let defocus_sample = Vec3::random_in_unit_disk();
                        let ray = Ray::look_at(
                            self.position
                                + defocus_sample.0 * self.defocus_disk.0
                                + defocus_sample.1 * self.defocus_disk.1,
                            sample,
                            rand::random(),
                        );

                        color += self.ray_color(root.clone(), ray, 0);
                    }
                }
                color /= samples;

                self.target.set_pixel(x, y, color.clamped());

                #[allow(clippy::cast_precision_loss)]
                if log {
                    print!(
                        "\rprogress: {:.2}%",
                        f64::from(y * self.target.width() + x) * 100.0
                            / self.target.pixel_count() as f64
                    );
                }
            }
        }
        if log {
            println!();
        }
    }

    fn ray_color(&self, root: Arc<dyn Hit>, ray: Ray, bounces: u32) -> Color {
        if bounces >= self.max_bounces {
            return Color::black();
        }

        if let Some(hit) = root.hit(&ray, Interval(BIAS..f64::INFINITY)) {
            let emitted = hit.material.emitted(hit.u, hit.v, &hit.point);

            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                emitted + attenuation * self.ray_color(root, scattered, bounces + 1)
            } else {
                emitted
            }
        } else {
            self.background.background(&ray)
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::camera::CameraBuilder;
    use crate::color::Color;
    use crate::image::Image;
    use crate::vec::Vec3;

    #[test]
    fn created_correctly() {
        let camera = CameraBuilder::new(1.0, 10.0, 90.0)
            .with_forward(Vec3(0.0, 0.0, -1.0))
            .build(Image::with_aspect_ratio(1, 1.0, Color::black()));

        assert_approx_eq!(Vec3, camera.viewport.origin, Vec3(-1.0, 1.0, -1.0));
        assert_approx_eq!(f64, camera.viewport.width, 2.0);
        assert_approx_eq!(
            Vec3,
            camera.viewport.edges.0.normalized(),
            Vec3(1.0, 0.0, 0.0)
        );
        assert_approx_eq!(
            Vec3,
            camera.viewport.edges.1.normalized(),
            Vec3(0.0, -1.0, 0.0)
        );
    }
}
