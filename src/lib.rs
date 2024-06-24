use::num::complex::Complex;
use::std::path::PathBuf;
use::chrono::{DateTime, TimeZone, Local};

pub fn run() {
    //figure out parameters
    //set up ouput and state
    //iterate over pixels
    //save image
}

pub fn iterations(iterations: u32, point: Complex<f64>) -> Option<Complex<f64>> {
    let mut i = 0;
    let mut z1 = Complex::new(0.0, 0.0);
    while i <= iterations {
        z1 = z1.powi(2) + point;
        i += 1;
    }
    return Some(z1)
}

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
