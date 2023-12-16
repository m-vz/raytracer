use std::path::Path;

use crate::color::Color;
use crate::hit::Hit;
use crate::image::Image;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec::Vec3;
use crate::viewport::Viewport;

pub struct Camera {
    position: Vec3,
    viewport: Viewport,
    target: Image,
    pub samples: u32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        forward: Vec3,
        up: Vec3,
        focal_length: f64,
        viewport_height: f64,
        target: Image,
        samples: u32,
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
            samples,
        }
    }

    pub fn render<P: AsRef<Path>>(&mut self, scene: &Scene, path: P) -> std::io::Result<()> {
        println!("starting render...");

        for i in 0..self.target.pixel_count() {
            let mut color = Color::black();

            for _ in 0..self.samples {
                let sample = self.viewport.pixel_sample(i);
                let ray = Ray::look_at(self.position, sample);

                color += Camera::ray_color(scene, ray);
            }
            color /= self.samples as f64;

            self.target.set_pixel_by_index(i, color.clamped());

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

    fn ray_color(scene: &Scene, ray: Ray) -> Color {
        if let Some(hit) = scene.hit(&ray, 0.0..f64::INFINITY) {
            0.5 * Color::new(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
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
            1,
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
