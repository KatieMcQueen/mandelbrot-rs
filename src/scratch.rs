extern crate image;
use::image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(32, 32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([y as u8,0,0]);
    }
    img.save("test.png").unwrap();
}
