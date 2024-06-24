use::mandelbrot::*;
use:: num::complex::Complex;
use::image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(1000, 1000);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x: u8 = ;
        let y: u8 = ;
        let val = iterations(256, point).unwrap();
        *pixel = Rgb([val, 0, 0]);
    }
    let path = build_path("images", "brot");
    println!("{}", path.display());
    img.save(path);



}
