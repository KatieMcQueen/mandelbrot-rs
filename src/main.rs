use ::image::{Rgb, RgbImage};
use ::mandelbrot::*;
use ::num::complex::Complex;

fn main() {
    let mut img = RgbImage::new(1000, 1000);

    println!("Building colormap!");
    let colors = build_colormap("data/color/romaO/romaO.lut").unwrap();
    println!("Built!");

    let zoom: f64 = 15625.0;
    let center: Complex<f64> = Complex::new(-0.761574, -0.0847596);

    let img = mandelbrot(center, zoom, img, colors);
    
    let path = build_path("images", "brot");
    println!("{}", path.display());
    img.save(path).expect("must be able to save");
}
