use crate::color::Color;
use crate::image::Image;

mod color;
mod image;
mod ray;
mod vec;

fn main() {
    let test_size = 400;
    let mut image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());

    let dimensions_float = (image.width() as f64, image.height() as f64);
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(
                x,
                y,
                Color::new(
                    x as f64 / dimensions_float.0,
                    y as f64 / dimensions_float.1,
                    0.0,
                ),
            );
        }
    }

    image.write_ppm("output/test.ppm").unwrap();
}
