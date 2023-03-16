extern crate image;

use image::{ImageFormat, GenericImageView};
use std::env;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 5 {
        eprintln!("Usage: {} [file|directory] [output_directory] [width] [height]", args[0]);
        std::process::exit(1);
    }
    let input_path = PathBuf::from(&args[1]);
    let output_dir = if args.len() >= 3 {
        Some(PathBuf::from(&args[2]))
    } else {
        None
    };
    let width = if args.len() >= 4 {
        Some(args[3].parse::<u32>().unwrap())
    } else {
        None
    };
    let height = if args.len() == 5 {
        Some(args[4].parse::<u32>().unwrap())
    } else {
        None
    };
    if input_path.is_dir() {
        for entry in fs::read_dir(input_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().unwrap_or_default() == "jfif" {
                convert_image(&path, &output_dir, width, height);
            }
        }
    } else {
        convert_image(&input_path, &output_dir, width, height);
    }
}

fn convert_image(input_path: &Path, output_dir: &Option<PathBuf>, width: Option<u32>, height: Option<u32>) {
    let output_path = if let Some(output_dir) = output_dir {
        output_dir.join(input_path.file_name().unwrap()).with_extension("png")
    } else {
        input_path.with_extension("png")
    };

    let input_file = File::open(input_path).unwrap();
    let input_reader = BufReader::new(input_file);

    let img = image::load(input_reader, ImageFormat::Jpeg).unwrap();

    let img = match (width,height) { 
      (Some(w),Some(h)) => img.resize(w,h,image::imageops::FilterType::Lanczos3),
      _ => img,
     };

   img.save(output_path).unwrap();
}