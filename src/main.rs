use image;
use rqrr::PreparedImage;

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

    let recipient_address_type = lines.next().expect("Missing recipient address type");
    assert!(recipient_address_type.eq("K") || recipient_address_type.eq("S"), "Only address types K and S are supported");

    let recipient_name = lines.next().expect("Missing recipient name");
    assert!(!recipient_name.is_empty(), "Recipient name is empty");

    let recipient_street_or_address_line_1 = lines.next().expect("Missing recipient street or address line 1");
    let recipient_building_number_or_address_line_2 = lines.next().expect("Missing recipient building number or address line 2");
    let recipient_postal_code = lines.next().expect("Missing recipient postal code");
    let recipient_town = lines.next().expect("Missing recipient town");
    let recipient_country = lines.next().expect("Missing recipient country");

    let recipient_address_line_1 = if recipient_address_type.eq("K") {
        format!("{recipient_street_or_address_line_1}")
    } else {
        format!("{recipient_street_or_address_line_1} {recipient_building_number_or_address_line_2}")
    };

    let recipient_address_line_2 = if recipient_address_type.eq("K") {
        format!("{recipient_building_number_or_address_line_2}")
    } else {
        format!("{recipient_postal_code} {recipient_town}")
    };

    lines.next();
    lines.next();
    lines.next();
    lines.next();
    lines.next();
    lines.next();
    lines.next();

    let amount = lines.next().expect("Missing amount");

    let currency = lines.next().expect("Missing currency");
    assert!(currency.eq("CHF") || currency.eq("EUR"), "Only CHF and EUR currencies are supported");

    let sender_address_type = lines.next().expect("Missing sender address type");
    let sender_name = lines.next().expect("Missing sender name");

    if !sender_address_type.is_empty() {
        assert!(sender_address_type.eq("K") || sender_address_type.eq("S"), "Only address types K and S are supported");
        assert!(!sender_name.is_empty(), "Sender name is empty");
    }

    let sender_street_or_address_line_1 = lines.next().expect("Missing sender street or address line 1");
    let sender_building_number_or_address_line_2 = lines.next().expect("Missing sender building number or address line 2");
    let sender_postal_code = lines.next().expect("Missing sender postal code");
    let sender_town = lines.next().expect("Missing sender town");
    let sender_country = lines.next().expect("Missing sender country");

    let sender_address_line_1 = if sender_address_type.eq("K") {
        format!("{sender_street_or_address_line_1}")
    } else {
        format!("{sender_street_or_address_line_1} {sender_building_number_or_address_line_2}")
    };

    let sender_address_line_2 = if sender_address_type.eq("K") {
        format!("{sender_building_number_or_address_line_2}")
    } else {
        format!("{sender_postal_code} {sender_town}")
    };

    let reference_type = lines.next().expect("Missing reference type");
    assert!(reference_type.eq("NON") || reference_type.eq("QRR") || reference_type.eq("SCOR"), "Only reference types NON, QRR and SCOR are supported");

    let reference = lines.next().expect("Missing reference");
    assert!((reference_type.eq("QRR") || reference_type.eq("SCOR")) && !reference.is_empty(), "Reference is empty");

    let message = lines.next().expect("Missing message");
    
    assert_eq!(lines.next(), Some("EPD"), "Missing trailing 'EPD'");


    let qr_data = QRData::new(
        iban.to_string(),
        recipient_address_type.to_string(),
        recipient_name.to_string(),
        recipient_address_line_1.to_string(),
        recipient_address_line_2.to_string(),
        recipient_country.to_string(),
        sender_address_type.to_string(),
        sender_name.to_string(),
        sender_address_line_1.to_string(),
        sender_address_line_2.to_string(),
        sender_country.to_string(),
        amount.to_string(),
        currency.to_string(),
        reference_type.to_string(),
        reference.to_string(),
        message.to_string(),
    );
    qr_data
}

#[derive(Debug)]
struct QRData {
    iban: String,
    recipient_address_type: String,
    recipient_name: String,
    recipient_address_line_1: String,
    recipient_address_line_2: String,
    recipient_country: String,
    sender_address_type: String,
    sender_name: String,
    sender_address_line_1: String,
    sender_address_line_2: String,
    sender_country: String,
    amount: String,
    currency: String,
    reference_type: String,
    reference: String,
    message: String,
}

impl QRData {
    fn new(iban: String,
           recipient_address_type: String,
           recipient_name: String,
           recipient_address_line_1: String,
           recipient_address_line_2: String,
           recipient_country: String,
           sender_address_type: String,
           sender_name: String,
           sender_address_line_1: String,
           sender_address_line_2: String,
           sender_country: String,
           amount: String,
           currency: String,
           reference_type: String,
           reference: String,
           message: String) -> Self {
        Self {
            iban,
            recipient_address_type,
            recipient_name,
            recipient_address_line_1,
            recipient_address_line_2,
            recipient_country,
            sender_address_type,
            sender_name,
            sender_address_line_1,
            sender_address_line_2,
            sender_country,
            amount,
            currency,
            reference_type,
            reference,
            message,
        }
    }
}

#[derive(Debug)]
struct Address {
    address_type: String,
    name: String,
    address_line_1: String,
    address_line_2: String,
    country: String,
}

impl Address {
    fn new(address_type: String, 
           name: String, 
           address_line_1: String, 
           address_line_2: String, 
           country: String) -> Self {
        Self {
            address_type,
            name,
            address_line_1,
            address_line_2,
            country,
        }
    }
}
