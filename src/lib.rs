//! # Simple program to read swiss invoices QR codes as pdf or png files and the relevant data
//!
//! This program reads QR codes from Swiss invoices and outputs the relevant data as JSON.
//!
//! See [the standard definition](https://www.six-group.com/de/products-services/banking-services/payment-standardization/standards/qr-bill.html#ig-qr-bill-v2.3)

pub mod models;
mod pdf_converter;
mod qr_parser;

use crate::models::qr_data::QRData;
use image;
use rayon::prelude::*;
use rqrr::PreparedImage;
use tempfile::tempdir;

pub fn get_qr_bill_data(file_path: String, fail_on_error: bool) -> Vec<QRData> {
    let tmp_dir = tempdir().expect("Error creating temporary directory");
    let images = match file_path.as_str() {
        input if input.ends_with(".pdf") => {
            pdf_converter::convert_to_png(&file_path, &tmp_dir.path())
        }
        input if input.ends_with(".png") || input.ends_with(".jpg") || input.ends_with(".jpeg") => {
            vec![image::open(&file_path).expect("Error loading image")]
        }
        _ => panic!("Unsupported file format"),
    };

    let all_qr_codes: Vec<_> = images
        .into_par_iter()
        .map(|img| {
            let mut img = PreparedImage::prepare(img.to_luma8());
            img.detect_grids()
                .into_par_iter()
                .filter_map(|result| result.decode().ok())
                .map(|(_, content)| qr_parser::get_qr_code_data(&content))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // check if there were any errors
    if fail_on_error && all_qr_codes.iter().any(|result| result.is_err()) {
        eprintln!("Error parsing QR codes");

        // print the errors
        for result in all_qr_codes {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        }

        std::process::exit(1);
    }

    let all_qr_codes: Vec<_> = all_qr_codes
        .into_iter()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect();

    return all_qr_codes;
}
