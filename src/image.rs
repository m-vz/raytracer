use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use crate::color::Color;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

#[allow(dead_code)]
impl Image {
    pub fn with_aspect_ratio(width: u32, aspect_ratio: f64, color: Color) -> Self {
        let height = (width as f64 / aspect_ratio) as u32;

        Self::with_dimensions(width, height, color)
    }

    pub fn with_dimensions(width: u32, height: u32, color: Color) -> Self {
        let height = height.max(1);
        let width = width.max(1);

        Self {
            width,
            height,
            data: vec![color; (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_count(&self) -> usize {
        self.data.len()
    }

    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn aspect(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: Color) {
        self.data[(y * self.width + x) as usize] = value;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.data[(y * self.width + x) as usize]
    }

    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        // create the directory and the file if they don't exist
        if let Some(directory) = path.as_ref().parent() {
            create_dir_all(directory)?;
        }
        let mut file = File::create(path)?;

        // write header
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;

        // write data
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = format!("{}\n", self.get_pixel(x, y));

                file.write_all(pixel.as_bytes())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::color::Color;

    use super::Image;

    #[test]
    fn create_with_aspect_ratio() {
        let image = Image::with_aspect_ratio(10, 16.0 / 9.0, Color::black());

        assert_eq!(image.width, 10);
        assert_eq!(image.height, 5);
        assert_approx_eq!(Color, image.get_pixel(4, 0), Color::black());
    }

    #[test]
    fn create_with_dimensions() {
        let color = Color::random();
        let image = Image::with_dimensions(10, 8, color);

        assert_eq!(image.width, 10);
        assert_eq!(image.height, 8);
        assert_approx_eq!(Color, image.get_pixel(3, 5), color);
    }

    #[test]
    fn pixels() {
        let color = Color::random();
        let mut image = Image::with_dimensions(3, 3, Color::black());

        assert_approx_eq!(Color, image.get_pixel(1, 1), Color::black());
        image.set_pixel(1, 1, color);
        assert_approx_eq!(Color, image.get_pixel(1, 1), color);
    }
}
