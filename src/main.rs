extern crate image;

use image::ImageFormat;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} INPUT", args[0]);
        std::process::exit(1);
    }
    let input_path = Path::new(&args[1]);
    let output_path = input_path.with_extension("png");

    let input_file = File::open(input_path).unwrap();
    let input_reader = BufReader::new(input_file);

    let img = image::load(input_reader, ImageFormat::Jpeg).unwrap();

    img.save(output_path).unwrap();
}