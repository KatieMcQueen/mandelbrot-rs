use::num::complex::Complex;
use::std::path::{ PathBuf, Path };
use::chrono::Local;
use::image::{Rgb, RgbImage};
use::std::fs;

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
            return Some(i)
        }    
        z1 = z1.powi(2) + point;
        i += 1;
    }
    None
}

//to make a color map we need to take in a file that describes it
//then we need to parse the values and save them into an array
//because colormaps may have variable lengths,
//we should encode both the length and the values into a struct

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
    Ok(ColorMap::new(data, contents.lines().count().try_into().unwrap()))
}

fn parse_line(line: &str) -> Result<Rgb<u8>, String> {
    let c: Vec<u8> = line.split(' ').map(|val| val.parse::<u8>().unwrap()).collect();
    Ok(Rgb([c[0], c[1], c[2]]))
}

pub fn transform() {

}

//creates a timestamped path using an output folder and a prefix
pub fn build_path(folder: &str, prefix: &str) -> PathBuf {
    let time = Local::now()
        .format("%F-%T");

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
