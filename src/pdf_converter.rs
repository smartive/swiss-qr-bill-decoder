use image::DynamicImage;
use std::process::{Command};
use std::path::Path;

pub fn convert_to_png(file_input: &str, tmp_dir: &Path) -> Vec<DynamicImage> {
    // use ghostscript to convert the PDF to a PNG
    gs_command(file_input, tmp_dir)
        .output()
        .expect("Error running Ghostscript. Is 'gs' installed?");

    // read images from temp directory
    let mut images = Vec::new();
    for i in 1.. {
        
        // let path = format!("{}/{:03}.png", tmp_dir.path(), i);
        let path = tmp_dir.join(format!("{:03}.png", i));
        if !Path::new(&path).exists() {
            break;
        }
        images.push(image::open(&path)
            .expect("Error loading image"));
    }
    
    return images;
}

fn gs_command<P, Q>(in_path: P, tmp_path: Q) -> Command
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
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
            format!(
                "-sOutputFile={}/%03d.png",
                tmp_path.as_ref().to_string_lossy().to_string()
            ).as_str(),
        ]
            .iter(),
    )
        .arg(in_path.as_ref().to_string_lossy().to_string())
        .arg(tmp_path.as_ref().to_string_lossy().to_string());
    cmd
}
