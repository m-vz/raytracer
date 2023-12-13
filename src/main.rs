use crate::image::{Image, Pixel};

mod image;

fn main() {
    let test_size = 640;
    let mut test_image = Image::black(test_size, test_size);

    for y in 0..test_size {
        for x in 0..test_size {
            test_image.set_pixel(
                x,
                y,
                Pixel((255 * x / test_size) as u8, (255 * y / test_size) as u8, 0),
            );
        }
    }

    test_image.write_ppm("output/test.ppm").unwrap();
}
