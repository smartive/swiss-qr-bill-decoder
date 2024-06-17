use clap::Parser;

/// Simple program to read swiss invoices QR codes as pdf or png files and the relevant data
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The file to read (pdf or png)
    #[arg(index = 1)]
    pub(crate) input: String,

    /// Pretty print the JSON output
    #[arg(short, long, default_value = "false")]
    pub(crate) pretty: bool,

    /// If true, the program will exit with an error code if any QR code could not be read
    #[arg(short, long, default_value = "false")]
    pub(crate) fail_on_error: bool,
}
