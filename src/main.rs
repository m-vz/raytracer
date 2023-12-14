use crate::color::Color;
use crate::image::Image;

mod color;
mod image;
mod ray;
mod vec;

fn main() {
    let test_size = 400;
    let mut test_image = Image::with_aspect_ratio(test_size, 16.0 / 9.0, Color::black());

    let test_size_float = test_size as f64;
    for y in 0..test_size {
        for x in 0..test_size {
            test_image.set_pixel(
                x,
                y,
                Color::new(x as f64 / test_size_float, y as f64 / test_size_float, 0.0),
            );
        }
    }

    test_image.write_ppm("output/test.ppm").unwrap();
}
