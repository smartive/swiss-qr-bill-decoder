use crate::models::args;
use clap::Parser;
use image;
use rqrr::PreparedImage;
use tempfile::tempdir;

mod models;
mod pdf_converter;
mod qr_parser;

fn main() {
    let args = args::Args::parse();
    let file_path = args.input;

    let tmp_dir = tempdir().expect("Error creating temporary directory");
    let images = match file_path.as_str() {
        input if input.ends_with(".pdf") => {
            pdf_converter::convert_to_png(&file_path, &tmp_dir.path())
        }
        input if input.ends_with(".png") => {
            vec![image::open(&file_path).expect("Error loading image")]
        }
        _ => panic!("Unsupported file format"),
    };

    let mut all_qr_codes = Vec::new();
    for img in images {
        let mut img = PreparedImage::prepare(img.to_luma8());
        img.detect_grids()
            .into_iter()
            .filter_map(|result| result.decode().ok())
            .map(|(_, content)| qr_parser::get_qr_code_data(&content))
            .for_each(|qr_data| all_qr_codes.push(qr_data));
    }

    // print the QR code data
    if args.pretty {
        serde_json::to_writer_pretty(std::io::stdout(), &all_qr_codes)
    } else {
        serde_json::to_writer(std::io::stdout(), &all_qr_codes)
    }
    .expect("Error writing JSON");
}
