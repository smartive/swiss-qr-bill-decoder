use std::path::Path;
use image;
use rqrr::PreparedImage;
use std::process::{Command, Stdio};

mod models;
mod qr_parser;

fn main() {
    let file_input = "data/minimal.pdf";

    // use ghostscript to convert the PDF to a PNG
    let png = gs_command(file_input)
        .output()
        .expect("Error running Ghostscript. Is 'gs' installed?");

    // read image from memory
    let img = image::load_from_memory_with_format(&png.stdout, image::ImageFormat::Png)
        .expect("Error loading image").to_luma8();

    let mut img = PreparedImage::prepare(img);

    let grids = img.detect_grids();
    if grids.len() == 0 {
        eprintln!("No QR codes found");
        return;
    }

    let qr_codes: Vec<_> = grids
        .into_iter()
        .map(|result| result.decode())
        .filter_map(|result| result.ok())
        .map(|(_, content)| content)
        .collect();

    for text_data in &qr_codes {
        let qr_data = qr_parser::get_qr_code_data(text_data);
        println!("{:?}", qr_data);
    }
}

pub fn gs_command<P>(in_path: P) -> Command
    where
        P: AsRef<Path>,
{
    let mut cmd = Command::new("gs");
    cmd.args(
        [
            "-q",
            "-dBATCH",
            "-dSAFER",
            "-dNOPAUSE",
            "-dFILTERTEXT",
            "-r300",
            "-sDEVICE=pngmono",
            "-sOutputFile=-"
        ]
            .iter(),
    )
        .arg(in_path.as_ref().to_string_lossy().to_string())
        .stdout(Stdio::piped());
    cmd
}