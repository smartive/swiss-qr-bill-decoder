use image::DynamicImage;
use std::path::Path;
use std::process::Command;

/// Convert a PDF file to a vector of PNG images
pub fn convert_to_png(file_input: &str, tmp_dir: &Path) -> Vec<DynamicImage> {
    // use ghostscript to convert the PDF to a PNG
    pdf_to_png(file_input, tmp_dir);

    // read images from temp directory
    (1..)
        .map(|i| tmp_dir.join(format!("{:03}.png", i)))
        .take_while(|path| path.exists())
        .map(|path| image::open(&path).expect("Error loading image"))
        .collect()
}

fn pdf_to_png<P, Q>(in_path: P, tmp_path: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let command = if cfg!(target_os = "windows") {
        "gswin64c"
    } else {
        "gs"
    };
    
    Command::new(command)
        .args(&[
            "-q",
            "-dBATCH",
            "-dSAFER",
            "-dNOPAUSE",
            "-dFILTERTEXT",
            "-r300",
            "-sDEVICE=pngmono",
            &format!(
                "-sOutputFile={}/%03d.png",
                tmp_path.as_ref().to_string_lossy()
            ),
        ])
        .arg(in_path.as_ref())
        .output()
        .expect("Error running Ghostscript. Is 'gs' installed?");
}
