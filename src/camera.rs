use std::path::Path;

use crate::color::Color;
use crate::hit::Hit;
use crate::image::Image;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec::Vec3;
use crate::viewport::Viewport;

const BIAS: f64 = 0.001;

pub struct Camera {
    pub position: Vec3,
    viewport: Viewport,
    target: Image,
    pub samples: u32,
    pub max_bounces: u32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        forward: Vec3,
        up: Vec3,
        focal_length: f64,
        viewport_height: f64,
        target: Image,
    ) -> Self {
        let viewport = Viewport::with_center(
            position + focal_length * forward,
            (viewport_height * target.aspect(), viewport_height),
            target.resolution(),
            forward.cross(&up),
            -up,
        );

        Self {
            position,
            viewport,
            target,
            samples: 10,
            max_bounces: 50,
        }
    }

    pub fn render<P: AsRef<Path>>(&mut self, scene: &Scene, path: P) -> std::io::Result<()> {
        println!("starting render...");

        for i in 0..self.target.pixel_count() {
            let mut color = Color::black();

            for _ in 0..self.samples {
                let sample = self.viewport.pixel_sample(i);
                let ray = Ray::look_at(self.position, sample);

                color += self.ray_color(scene, ray, 0);
            }
            color /= self.samples as f64;

            self.target
                .set_pixel_by_index(i, color.clamped().to_gamma_space());

            print!(
                "\rprogress: {:.2}%",
                i as f64 * 100.0 / self.target.pixel_count() as f64
            );
        }

        println!("\nwriting file...");
        let result = self.target.write_ppm(path);

        println!("\ndone");
        result
    }

    fn ray_color(&self, scene: &Scene, ray: Ray, bounces: u32) -> Color {
        if bounces >= self.max_bounces {
            return Color::black();
        }

        if let Some(hit) = scene.hit(&ray, BIAS..f64::INFINITY) {
            0.5 * self.ray_color(
                scene,
                Ray {
                    origin: hit.point,
                    direction: hit.normal + Vec3::random_unit_vector(),
                },
                bounces + 1,
            )
        } else {
            Camera::background(ray)
        }
    }

    fn background(ray: Ray) -> Color {
        let a = 0.5 * (ray.normalized().direction.y() + 1.0);

        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
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
        let camera = Camera::new(
            Vec3(0.0, 0.0, 0.0),
            Vec3(0.0, 0.0, -1.0),
            Vec3(0.0, 1.0, 0.0),
            1.0,
            1.0,
            Image::with_aspect_ratio(1, 1.0, Color::black()),
        );

        assert_approx_eq!(Vec3, camera.viewport.origin(), Vec3(-0.5, 0.5, -1.0));
        assert_approx_eq!(f64, camera.viewport.width(), 1.0);
        assert_approx_eq!(
            Vec3,
            camera.viewport.edges().0.normalized(),
            Vec3(1.0, 0.0, 0.0)
        );
        assert_approx_eq!(
            Vec3,
            camera.viewport.edges().1.normalized(),
            Vec3(0.0, -1.0, 0.0)
        );
    }
}
