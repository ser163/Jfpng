extern crate image;
use image::{ImageFormat};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Input file or directory
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Output directory
    #[structopt(parse(from_os_str))]
    output_dir: Option<PathBuf>,
    /// Width of the output image
    width: Option<u32>,
    /// Height of the output image
    height: Option<u32>,
}

fn main() {
    let opt = Opt::from_args();
    
    if opt.input.is_dir() {
        for entry in fs::read_dir(opt.input).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().unwrap_or_default() == "jfif" {
                convert_image(&path, &opt.output_dir, opt.width, opt.height);
            }
        }
    } else {
        convert_image(&opt.input, &opt.output_dir, opt.width,opt.height);
    }
}

fn convert_image(input_path: &Path,output_dir: &Option<PathBuf>,width: Option<u32>,height: Option<u32>) {
    
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