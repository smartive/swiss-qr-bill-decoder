//! # Simple program to read Swiss invoice QR codes from PDF or PNG files and extract relevant data
//!
//! This program reads QR codes from Swiss invoices and outputs the relevant data as JSON.
//!
//! See [the standard definition](https://www.six-group.com/de/products-services/banking-services/payment-standardization/standards/qr-bill.html#ig-qr-bill-v2.3)

pub mod models;
mod pdf_converter;
mod qr_parser;

use crate::models::qr_data::QRData;
use image::DynamicImage;
use rayon::prelude::*;
use rqrr::PreparedImage;
use std::path::Path;
use tempfile::tempdir;

pub fn get_qr_bill_data(file_path: &str, fail_on_error: bool) -> Vec<QRData> {
    let tmp_dir = tempdir().expect("Failed to create temporary directory");

    let images = load_images(file_path, tmp_dir.path());

    let qr_data_results: Vec<_> = images
        .into_par_iter()
        .flat_map(|img| extract_qr_data(&img))
        .collect();

    handle_errors(&qr_data_results, fail_on_error);

    qr_data_results.into_iter().filter_map(Result::ok).collect()
}

fn load_images(file_path: &str, tmp_dir_path: &Path) -> Vec<DynamicImage> {
    match file_path.to_lowercase().as_str() {
        input if input.ends_with(".pdf") => pdf_converter::convert_to_png(file_path, tmp_dir_path),
        input if input.ends_with(".png") || input.ends_with(".jpg") || input.ends_with(".jpeg") => {
            vec![image::open(file_path).expect("Failed to load image")]
        }
        _ => panic!("Unsupported file format"),
    }
}

fn extract_qr_data(img: &DynamicImage) -> Vec<Result<QRData, String>> {
    PreparedImage::prepare(img.to_luma8())
        .detect_grids()
        .into_par_iter()
        .filter_map(|grid| grid.decode().ok())
        .map(|(_, content)| qr_parser::get_qr_code_data(&content))
        .collect()
}

fn handle_errors(results: &[Result<QRData, String>], fail_on_error: bool) {
    if fail_on_error && results.iter().any(Result::is_err) {
        eprintln!("Error parsing QR codes");

        for result in results {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        }

        std::process::exit(1);
    }
}
