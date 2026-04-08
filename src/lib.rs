//! # Simple program to read Swiss invoice QR codes from PDF or PNG files and extract relevant data
//!
//! This program reads QR codes from Swiss invoices and outputs the relevant data as JSON.
//!
//! See [the standard definition](https://www.six-group.com/de/products-services/banking-services/payment-standardization/standards/qr-bill.html#ig-qr-bill-v2.3)

pub mod models;
mod pdf_converter;
mod qr_parser;

use crate::models::qr_data::QRData;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GrayImage;
use rayon::prelude::*;
use rqrr::PreparedImage;
use std::path::Path;
use tempfile::tempdir;

const CONTRAST: f32 = 40.0;
const THRESHOLD: u8 = 180;

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
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

    match extension.as_deref() {
        Some("pdf") => pdf_converter::convert_to_png(file_path, tmp_dir_path),
        Some("png" | "jpg" | "jpeg") => vec![image::open(file_path).expect("Failed to load image")],
        _ => panic!("Unsupported file format"),
    }
}

fn extract_qr_data(img: &DynamicImage) -> Vec<Result<QRData, String>> {
    let original = img.to_luma8();

    let decoded = decode_qr_from_grayscale(original.clone());
    if !decoded.is_empty() {
        return decoded;
    }

    let contrast = image::imageops::contrast(&original, CONTRAST);
    let decoded = decode_qr_from_grayscale(contrast);
    if !decoded.is_empty() {
        return decoded;
    }

    let thresholded = threshold_image(&original, THRESHOLD);
    let decoded = decode_qr_from_grayscale(thresholded);
    if !decoded.is_empty() {
        return decoded;
    }

    let inverted = invert_image(&original);
    let decoded = decode_qr_from_grayscale(inverted.clone());
    if !decoded.is_empty() {
        return decoded;
    }

    let thresholded_inverted = threshold_image(&inverted, THRESHOLD);
    let decoded = decode_qr_from_grayscale(thresholded_inverted);
    if !decoded.is_empty() {
        return decoded;
    }

    for candidate in [
        image::imageops::rotate90(&original),
        image::imageops::rotate180(&original),
        image::imageops::rotate270(&original),
    ] {
        let decoded = decode_qr_from_grayscale(candidate);
        if !decoded.is_empty() {
            return decoded;
        }
    }

    for candidate in [
        image::imageops::resize(
            &original,
            original.width() * 2,
            original.height() * 2,
            FilterType::Nearest,
        ),
        image::imageops::resize(
            &original,
            (original.width() / 2).max(1),
            (original.height() / 2).max(1),
            FilterType::Triangle,
        ),
    ] {
        let decoded = decode_qr_from_grayscale(candidate);
        if !decoded.is_empty() {
            return decoded;
        }
    }

    Vec::new()
}

fn decode_qr_from_grayscale(img: GrayImage) -> Vec<Result<QRData, String>> {
    PreparedImage::prepare(img)
        .detect_grids()
        .into_iter()
        .filter_map(|grid| grid.decode().ok())
        .map(|(_, content)| qr_parser::get_qr_code_data(&content))
        .collect()
}

fn invert_image(img: &GrayImage) -> GrayImage {
    let mut inverted = img.clone();
    image::imageops::invert(&mut inverted);
    inverted
}

fn threshold_image(img: &GrayImage, threshold: u8) -> GrayImage {
    let mut thresholded = img.clone();
    for pixel in thresholded.pixels_mut() {
        pixel[0] = if pixel[0] > threshold { 255 } else { 0 };
    }
    thresholded
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
