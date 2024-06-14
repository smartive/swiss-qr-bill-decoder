//! # Simple program to read swiss invoices QR codes as pdf or png files and the relevant data
//!
//! This program reads QR codes from Swiss invoices and outputs the relevant data as JSON.
//!
//! See [the standard definition](https://www.six-group.com/de/products-services/banking-services/payment-standardization/standards/qr-bill.html#ig-qr-bill-v2.3)

use clap::Parser;
use swiss_qr_bill_decoder_lib::get_qr_bill_data;
 mod args;

fn main() {
    let args = args::Args::parse();
    let all_qr_codes: Vec<_> = get_qr_bill_data(args.input, args.fail_on_error);

    // send the QR code data to stdout
    if args.pretty {
        serde_json::to_writer_pretty(std::io::stdout(), &all_qr_codes)
    } else {
        serde_json::to_writer(std::io::stdout(), &all_qr_codes)
    }
    .expect("Error writing JSON");
}
