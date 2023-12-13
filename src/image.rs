use std::fmt::Display;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub struct Pixel(pub u8, pub u8, pub u8);

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn black(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: Pixel) {
        let i = self.index_at(x, y);

        self.data[i] = value.0;
        self.data[i + 1] = value.1;
        self.data[i + 2] = value.2;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let i = self.index_at(x, y);

        Pixel(self.data[i], self.data[i + 1], self.data[i + 2])
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

    fn index_at(&self, x: u32, y: u32) -> usize {
        3 * (y * self.width + x) as usize
    }
}
