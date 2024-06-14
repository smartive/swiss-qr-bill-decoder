use image;
use rqrr::PreparedImage;
use crate::models::address::Address;
use crate::models::qr_data::QRData;

mod models;

fn main() {
    let file_name = "test_image.png";

    let img = image::open(file_name).expect("Error loading image").to_luma8();

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
        let qr_data = get_qr_code_data(text_data);
        println!("{:?}", qr_data);
    }
}

fn get_qr_code_data(text: &String) -> QRData {
    for (i, line) in text.lines().enumerate() {
        println!("[{i}] {line}");
    }

    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("SPC"), "First line is not 'SPC'");
    assert_eq!(lines.next(), Some("0200"), "Only version 0200 is supported");
    assert_eq!(lines.next(), Some("1"), "Only coding type 1 (UTF-8) is supported");

    let iban = lines.next().expect("Missing IBAN");
    assert!(iban.starts_with("CH") || iban.starts_with("LI"), "Only CH and LI IBANs are supported");

    let recipient_address = Address::new(
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
    );

    // skip 7 lines
    for _ in 0..7 {
        let _ = lines.next();
    }

    let amount = lines.next().expect("Missing amount");

    let currency = lines.next().expect("Missing currency");
    assert!(currency.eq("CHF") || currency.eq("EUR"), "Only CHF and EUR currencies are supported");

    let address_type = lines.next().expect("Missing address type");
    let sender_address = if !address_type.is_empty() {
        Some(Address::new(
            Some(address_type),
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
            lines.next(),
        ))
    } else {
        None
    };

    let reference_type = lines.next().expect("Missing reference type");
    assert!(reference_type.eq("NON") || reference_type.eq("QRR") || reference_type.eq("SCOR"), "Only reference types NON, QRR and SCOR are supported");

    let reference = lines.next().expect("Missing reference");
    assert!((reference_type.eq("QRR") || reference_type.eq("SCOR")) && !reference.is_empty(), "Reference is empty");

    let message = lines.next().expect("Missing message");

    assert_eq!(lines.next(), Some("EPD"), "Missing trailing 'EPD'");

    let qr_data = QRData::new(
        iban.to_string(),
        recipient_address,
        sender_address,
        amount.to_string(),
        currency.to_string(),
        reference_type.to_string(),
        reference.to_string(),
        message.to_string(),
    );
    qr_data
}
