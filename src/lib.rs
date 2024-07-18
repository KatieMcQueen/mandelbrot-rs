use ::chrono::Local;
use ::image::{Rgb, RgbImage};
use ::num::complex::Complex;
use ::std::fs;
use ::std::path::{Path, PathBuf};


pub fn run() {
    //figure out parameters
    //set up ouput and state
    //iterate over pixels
    //save image
}

//the central iteration function
//in a more advanced version maybe this should take a closure that can generate
//different fractals flexibly
pub fn iterations(iterations: u32, point: Complex<f64>) -> Option<u32> {
    let mut i = 0;
    let mut z1 = Complex::new(0.0, 0.0);
    while i <= iterations {
        if z1.norm_sqr() > 4.0 {
            return Some(i);
        }
        z1 = z1.powi(2) + point;
        i += 1;
    }
    None
}

pub fn mandelbrot(center: Complex<f64>, zoom: f64, mut img: RgbImage, colors: ColorMap) -> RgbImage {
    //set up values for the transform
    let zoom = 1.0 / zoom;
    let (width, height) = img.dimensions();

    let width_complex = (2.0 * width as f64) / height as f64;

    let mut top_left = Complex::new(-width_complex, 2.0);
    let mut bottom_right = Complex::new(width_complex, -2.0);

    //apply the zoom to both corners, then add the center to both
    //the length of the subsection of the complex plane can be derived 
    //by finding the delta of the top and bottom corners
    //calculate an increment for both image dimensions 
    //by dividing the length in that axis by the number of pixels
    //then to obtain the complex coordinate at that image coordinate multiply each axis by the
    //calculated increment value
    
    top_left = top_left.scale(zoom);
    bottom_right = bottom_right.scale(zoom);

    top_left += center;
    bottom_right += center;

    let size = top_left - bottom_right;
    let size = Complex::new(size.re.abs(), size.im.abs());

    let x_inc = size.re / width as f64;
    let y_inc = size.im / height as f64;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let re = top_left.re + (x as f64 * x_inc);
        let im = top_left.im - (y as f64 * y_inc);
        let point = Complex::new(re, im);

        match iterations(1023, point) {
            Some(val) => *pixel = colors.assign(val),
            None => *pixel = Rgb([0, 0, 0]),
        }
    }
    img
}

pub struct ColorMap {
    //with the data members private we need to provide access through getter functions
    data: Vec<Rgb<u8>>,
    length: u32,
}

impl ColorMap {
    pub fn new(data: Vec<Rgb<u8>>, length: u32) -> Self {
        ColorMap {
            data: data,
            length: length,
        }
    }
    pub fn assign(&self, iterations: u32) -> Rgb<u8> {
        let val = iterations % self.length;
        self.data[val as usize]
    }
}

//maybe this should be rolled into the color map struct?
pub fn build_colormap<P: AsRef<Path>>(path: P) -> Result<ColorMap, String> {
    let contents = fs::read_to_string(path).unwrap();
    let mut data: Vec<Rgb<u8>> = Vec::new();
    for line in contents.lines() {
        data.push(parse_line(line).unwrap());
    }
    Ok(ColorMap::new(
        data,
        contents.lines().count().try_into().unwrap(),
    ))
}

fn parse_line(line: &str) -> Result<Rgb<u8>, String> {
    let c: Vec<u8> = line
        .split(' ')
        .map(|val| val.parse::<u8>().unwrap())
        .collect();
    Ok(Rgb([c[0], c[1], c[2]]))
}

pub fn transform() {}

//creates a timestamped path using an output folder and a prefix
pub fn build_path(folder: &str, prefix: &str) -> PathBuf {
    let time = Local::now().format("%F-%T");

    let file = format!("{folder}/{prefix}-{time}.png");
    let path = PathBuf::from(&file);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
