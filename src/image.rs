use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use crate::color::Color;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Image {
    pub fn black(width: u32, height: u32) -> Self {
        Image::uniform(width, height, Color::black())
    }

    pub fn uniform(width: u32, height: u32, color: Color) -> Self {
        Self {
            width,
            height,
            data: vec![color; (width * height) as usize],
        }
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
                let postfix = if x == self.width - 1 { "\n" } else { " " };
                let pixel = format!("{}{}", self.get_pixel(x, y), postfix);

                file.write_all(pixel.as_bytes())?;
            }
        }

        Ok(())
    }
}
