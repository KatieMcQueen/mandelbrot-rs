use ::image::{RgbImage};
use ::mandelbrot::*;
use ::num::complex::Complex;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(allow_negative_numbers(true))]
    real: f64,
    #[arg(allow_negative_numbers(true))]
    imaginary: f64,
    zoom: f64,

    #[arg(long)]
    width: Option<u32>,
    #[arg(long)]
    height: Option<u32>,

    ///provide a path to a colormap file
    #[arg(short, long)]
    colors: Option<std::path::PathBuf>,

    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut dimensions = (2160, 1440);

    match(cli.height, cli.width) {
        (Some(y), Some(x)) => { dimensions = (x, y) },
        (Some(y), None) => { dimensions.1 = y },
        (None, Some(x)) => { dimensions.0 = x },
        (None, None) => {},
    }

    let img = RgbImage::new(dimensions.0, dimensions.1);

    

    println!("Building colormap!");
    let colors = build_colormap("data/color/romaO/romaO.lut").unwrap();
    println!("Built!");

    let zoom: f64 = cli.zoom;
    let center: Complex<f64> = Complex::new(cli.real, cli.imaginary);

    let img = mandelbrot(center, zoom, img, colors);
    
    let path = build_path("images", "brot");
    println!("{}", path.display());
    img.save(path).expect("must be able to save");
}
