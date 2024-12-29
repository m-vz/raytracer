use crate::vec::Vec3;

#[derive(Clone)]
pub struct Viewport {
    /// Width of the viewport in world units.
    pub width: f64,
    /// Height of the viewport in world units.
    pub height: f64,
    /// Location of the upper left corner of the viewport.
    pub origin: Vec3,
    /// Vectors pointing along the horizontal and vertical axes of the viewport.
    pub edges: (Vec3, Vec3),
    /// Vectors pointing from a pixel to the neighbour to its right and below it.
    pixel_size: (Vec3, Vec3),
}

#[allow(dead_code)]
impl Viewport {
    pub fn with_center(
        center: Vec3,
        size: (f64, f64),
        resolution: (u32, u32),
        right: Vec3,
        down: Vec3,
    ) -> Self {
        let origin = center - 0.5 * (size.0 * right + size.1 * down);

        Self::with_origin(origin, size, resolution, right, down)
    }

    pub fn with_origin(
        origin: Vec3,
        size: (f64, f64),
        resolution: (u32, u32),
        right: Vec3,
        down: Vec3,
    ) -> Self {
        let pixel_size = (size.0 / resolution.0 as f64, size.1 / resolution.1 as f64);

        Self {
            width: size.0,
            height: size.1,
            origin,
            edges: (size.0 * right, size.1 * down),
            pixel_size: (pixel_size.0 * right, pixel_size.1 * down),
        }
    }

    pub fn pixel_sample(
        &self,
        x: u32,
        y: u32,
        sample_x: u32,
        sample_y: u32,
        subpixel_scale: f64,
    ) -> Vec3 {
        self.origin
            + self.pixel_size.0
                * (x as f64 + subpixel_scale * (sample_x as f64 + rand::random::<f64>()))
            + self.pixel_size.1
                * (y as f64 + subpixel_scale * (sample_y as f64 + rand::random::<f64>()))
    }

    pub const fn pixel_size(&self) -> (Vec3, Vec3) {
        self.pixel_size
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::vec::Vec3;

    use super::Viewport;

    #[test]
    fn created_correctly() {
        let viewport = Viewport::with_origin(
            Vec3(-1.0, 0.5, -1.0),
            (2.0, 1.0),
            (10, 10),
            Vec3(1.0, 0.0, 0.0),
            Vec3(0.0, -1.0, 0.0),
        );

        assert_approx_eq!(Vec3, viewport.edges.0, Vec3(2.0, 0.0, 0.0));
        assert_approx_eq!(Vec3, viewport.edges.1, Vec3(0.0, -1.0, 0.0));
        assert_approx_eq!(Vec3, viewport.pixel_size.0, Vec3(0.2, 0.0, 0.0));
        assert_approx_eq!(Vec3, viewport.pixel_size.1, Vec3(0.0, -0.1, 0.0));
    }

    #[test]
    fn with_center_translated_correctly() {
        let viewport_with_center = Viewport::with_center(
            Vec3(0.0, 0.0, -1.0),
            (1.0, 1.0),
            (10, 10),
            Vec3(1.0, 0.0, 0.0),
            Vec3(0.0, -1.0, 0.0),
        );
        let viewport_with_origin = Viewport::with_origin(
            Vec3(-0.5, 0.5, -1.0),
            (1.0, 1.0),
            (10, 10),
            Vec3(1.0, 0.0, 0.0),
            Vec3(0.0, -1.0, 0.0),
        );

        assert_approx_eq!(
            Vec3,
            viewport_with_center.origin,
            viewport_with_origin.origin
        );
    }
}
