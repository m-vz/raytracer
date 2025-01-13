use std::ffi::OsStr;
use std::fs::{self, create_dir_all, File};
use std::io::{BufReader, Write};
use std::path::Path;

use egui::ColorImage;
use image::codecs::hdr::HdrDecoder;
use image::{ImageBuffer, Rgb};

use crate::color::Color;
use crate::math;

#[derive(Debug)]
pub enum ImageError {
    #[allow(dead_code)] // can be removed as soon as this error is logged correctly
    IOError(std::io::Error),
    #[allow(dead_code)] // can be removed as soon as this error is logged correctly
    SaveError(image::ImageError),
    AveragingZeroImages,
    DimensionsMismatch,
}

impl From<std::io::Error> for ImageError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

#[allow(dead_code)]
impl Image {
    pub fn with_aspect_ratio(width: u32, aspect_ratio: f64, color: Color) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let height = (f64::from(width) / aspect_ratio) as u32;

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

    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let width: u32;
        let height: u32;
        let data = if path.as_ref().extension() == Some(OsStr::new("hdr")) {
            let file = File::open(&path).expect("Failed to open image");
            let decoder = HdrDecoder::new(BufReader::new(file)).expect("Failed to read hdr image");
            width = decoder.metadata().width;
            height = decoder.metadata().height;
            decoder
                .read_image_hdr()
                .expect("Failed to read image")
                .into_iter()
                .map(|pixel| {
                    Color::new(
                        f64::from(pixel.0[0]),
                        f64::from(pixel.0[1]),
                        f64::from(pixel.0[2]),
                    )
                })
                .collect()
        } else {
            let image = image::open(path).expect("Failed to open image");
            width = image.width();
            height = image.height();
            image
                .into_rgb8()
                .enumerate_pixels()
                .map(|(_, _, pixel)| Color::from(*pixel))
                .collect()
        };

        Self {
            width,
            height,
            data,
        }
    }

    pub fn average(images: &Vec<Self>) -> Result<Self, ImageError> {
        if images.is_empty() {
            return Err(ImageError::AveragingZeroImages);
        }
        let pixel_count = images[0].data.len();
        if images.iter().any(|image| image.data.len() != pixel_count) {
            return Err(ImageError::DimensionsMismatch);
        }

        let mut data = vec![Color::black(); pixel_count];

        #[allow(clippy::cast_precision_loss)]
        data.iter_mut().enumerate().for_each(|(i, pixel)| {
            for image in images {
                *pixel += image.data[i];
            }
            *pixel /= images.len() as f64;
        });

        Ok(Self {
            width: images[0].width,
            height: images[0].height,
            data,
        })
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_count(&self) -> usize {
        self.data.len()
    }

    pub const fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn aspect(&self) -> f64 {
        f64::from(self.width) / f64::from(self.height)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: Color) {
        self.data[(y * self.width + x) as usize] = value;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.data[(y * self.width + x) as usize]
    }

    pub fn get_pixel_by_uv(&self, u: f64, v: f64) -> Color {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        self.get_pixel(
            (math::clamp_repeating(u) * f64::from(self.width)) as u32,
            ((1.0 - math::clamp_repeating(v)) * f64::from(self.height)) as u32,
        )
    }

    pub fn write_ppm<P: AsRef<Path>>(
        &self,
        path: P,
        to_gamma_space: bool,
    ) -> Result<(), ImageError> {
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
                let pixel = format!(
                    "{}\n",
                    if to_gamma_space {
                        self.get_pixel(x, y).to_gamma_space()
                    } else {
                        self.get_pixel(x, y)
                    }
                );

                file.write_all(pixel.as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn write_png<P: AsRef<Path>>(
        &self,
        path: P,
        to_gamma_space: bool,
    ) -> Result<(), ImageError> {
        let image: ImageBuffer<Rgb<u8>, _> =
            ImageBuffer::from_fn(self.width, self.height, |x, y| {
                if to_gamma_space {
                    self.get_pixel(x, y).to_gamma_space().into()
                } else {
                    self.get_pixel(x, y).into()
                }
            });

        path.as_ref().parent().map(fs::create_dir_all);
        image.save(path).map_err(ImageError::SaveError)
    }
}

impl From<&Image> for ColorImage {
    fn from(value: &Image) -> Self {
        Self::from_rgb(
            [value.width() as usize, value.height() as usize],
            &value
                .data
                .iter()
                .flat_map(Color::as_bytes)
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::color::Color;

    use super::Image;

    #[test]
    fn create_with_aspect_ratio() {
        let image = Image::with_aspect_ratio(10, 16.0 / 9.0, Color::black());

        assert_eq!(image.width, 10);
        assert_eq!(image.height, 5);
        assert_abs_diff_eq!(image.get_pixel(4, 0), Color::black());
    }

    #[test]
    fn create_with_dimensions() {
        let color = Color::random();
        let image = Image::with_dimensions(10, 8, color);

        assert_eq!(image.width, 10);
        assert_eq!(image.height, 8);
        assert_abs_diff_eq!(image.get_pixel(3, 5), color);
    }

    #[test]
    fn pixels() {
        let color = Color::random();
        let mut image = Image::with_dimensions(3, 3, Color::black());

        assert_abs_diff_eq!(image.get_pixel(1, 1), Color::black());
        image.set_pixel(1, 1, color);
        assert_abs_diff_eq!(image.get_pixel(1, 1), color);
    }
}
