use::mandelbrot::*;
use:: num::complex::Complex;
use::image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(1000, 1000);
    let (w, h) = img.dimensions();

    println!("Building colormap!");
    let colors = build_colormap("data/color/bamO/bamO.lut").unwrap();
    println!("Built!");

    let zoom: f64 = 1;
    let center: Complex = Complex::new(0,0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        //this code needs to be refactored elsewhere
        //get width and hight
        //convert width to real component
        //convert hight to imaginary component
        //build complex representation
        let re = (x as f64 / w as f64 -0.5) * 2.0;
        let im = (y as f64 / h as f64 -0.5) * 2.0;
        let point = Complex::new(re, im);
        
        match iterations(255, point) {
            Some(val) => *pixel = colors.assign(val),
            None => *pixel = Rgb([0, 0, 0]),
        }
    }
    let path = build_path("images", "brot");
    println!("{}", path.display());
    img.save(path).expect("must be able to save");



}
