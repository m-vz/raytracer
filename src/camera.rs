use std::path::Path;

use crate::color::Color;
use crate::image::Image;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::viewport::Viewport;

pub struct Camera {
    position: Vec3,
    viewport: Viewport,
    target: Image,
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
            (viewport_height, viewport_height * target.aspect()),
            target.resolution(),
            forward.cross(&up),
            -up,
        );

        Self {
            position,
            viewport,
            target,
        }
    }

    pub fn render_to<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        for (i, pixel) in self.viewport.pixels().iter().enumerate() {
            let ray = Ray::look_at(self.position, *pixel);

            self.target.set_pixel_by_index(i, Camera::ray_color(ray));
        }

        self.target.write_ppm(path)
    }

    fn ray_color(ray: Ray) -> Color {
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
