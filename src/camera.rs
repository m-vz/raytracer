use std::io;
use std::path::Path;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Instant;

use crate::color::Color;
use crate::hit::Hit;
use crate::image::{Image, ImageError};
use crate::math;
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
pub struct Camera {
    pub position: Vec3,
    viewport: Viewport,
    defocus_disk: (Vec3, Vec3),
    target: Image,
    pub samples: u32,
    pub max_bounces: u32,
    pub background: Color,
}

impl Camera {
    pub fn look_at(
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        focus_distance: f64,
        defocus_angle: f64,
        fov: f64,
        target: Image,
        background: Color,
    ) -> Self {
        Self::face(
            position,
            position.look_at(&look_at),
            up,
            focus_distance,
            defocus_angle,
            fov,
            target,
            background,
        )
    }

    pub fn face(
        position: Vec3,
        mut forward: Vec3,
        up: Vec3,
        focus_distance: f64,
        defocus_angle: f64,
        fov: f64,
        target: Image,
        background: Color,
    ) -> Self {
        forward.normalize();
        let right = forward.cross(&up.normalized());
        let up = right.cross(&forward);

        let h = (math::deg_to_rad(fov) / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;

        let viewport = Viewport::with_center(
            position + focus_distance * forward,
            (viewport_height * target.aspect(), viewport_height),
            target.resolution(),
            right,
            -up,
        );

        let defocus_radius = focus_distance * math::deg_to_rad(defocus_angle / 2.0).tan();

        Self {
            position,
            viewport,
            defocus_disk: (right * defocus_radius, up * defocus_radius),
            target,
            samples: 9,
            max_bounces: 50,
            background,
        }
    }

    pub fn render_and_save<P: AsRef<Path>>(
        &mut self,
        root: Arc<dyn Hit>,
        path: P,
        num_threads: u32,
    ) -> Result<(), CameraError> {
        println!("starting render...");
        let t = Instant::now();

        let samples_per_thread_sqrt = (self.samples as f64 / num_threads as f64).sqrt() as u32;

        if num_threads > 1 {
            let mut threads = Vec::with_capacity(num_threads as usize);
            for i in 0..num_threads {
                let mut thread_camera = self.clone();
                let thread_root = root.clone();

                threads.push(spawn(move || {
                    thread_camera.render(thread_root, samples_per_thread_sqrt, i == 0);
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
            self.render(root, samples_per_thread_sqrt, true);
        }

        println!("writing file...");
        if let Err(ImageError::IOError(error)) = self.target.write_png(path, true) {
            return Err(error.into());
        }

        println!("done in {}ms", t.elapsed().as_millis());
        Ok(())
    }

    fn render(&mut self, root: Arc<dyn Hit>, samples_sqrt: u32, log: bool) {
        let subpixel_scale = 1.0 / samples_sqrt as f64;
        let samples = samples_sqrt * samples_sqrt;

        for y in 0..self.target.height() {
            for x in 0..self.target.width() {
                let mut color = Color::black();

                for sample_y in 0..samples_sqrt {
                    for sample_x in 0..samples_sqrt {
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
                color /= samples as f64;

                self.target.set_pixel(x, y, color.clamped());

                if log {
                    print!(
                        "\rprogress: {:.2}%",
                        (y * self.target.width() + x) as f64 * 100.0
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
            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                attenuation * self.ray_color(root, scattered, bounces + 1)
            } else {
                Color::black()
            }
        } else {
            self.background
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::color::Color;
    use crate::image::Image;
    use crate::vec::Vec3;

    use super::Camera;

    #[test]
    fn created_correctly() {
        let camera = Camera::face(
            Vec3(0.0, 0.0, 0.0),
            Vec3(0.0, 0.0, -1.0),
            Vec3(0.0, 1.0, 0.0),
            1.0,
            10.0,
            90.0,
            Image::with_aspect_ratio(1, 1.0, Color::black()),
            Color::black(),
        );

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
