use crate::models::address::Address;
use crate::models::qr_data::QRData;

pub fn get_qr_code_data(text: &String) -> Result<QRData, String> {
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("SPC"), "First line is not 'SPC'");
    assert_eq!(lines.next(), Some("0200"), "Only version 0200 is supported");
    assert_eq!(
        lines.next(),
        Some("1"),
        "Only coding type 1 (UTF-8) is supported"
    );

    let iban = lines.next().expect("Missing IBAN");
    assert!(
        iban.starts_with("CH") || iban.starts_with("LI"),
        "Only CH and LI IBANs are supported"
    );

    let recipient_address = Address::new(
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
    )?;

    // skip 7 lines
    for _ in 0..7 {
        let _ = lines.next();
    }

    let amount = lines.next().expect("Missing amount").trim();
    let currency = lines.next().expect("Missing currency");
    assert!(
        currency.eq("CHF") || currency.eq("EUR"),
        "Only CHF and EUR currencies are supported"
    );

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
        )?)
    } else {
        // skip 7 lines
        for _ in 0..6 {
            let _ = lines.next();
        }
        None
    };

    let reference_type = lines.next().expect("Missing reference type");
    assert!(
        reference_type.eq("NON") || reference_type.eq("QRR") || reference_type.eq("SCOR"),
        "Only reference types NON, QRR and SCOR are supported"
    );

    let reference = lines.next().expect("Missing reference").trim();
    if reference_type.eq("QRR") || reference_type.eq("SCOR") {
        assert!(!reference.is_empty(), "Reference is empty");
    }

    let message = lines.next().expect("Missing message").trim();

    assert_eq!(lines.next(), Some("EPD"), "Missing trailing 'EPD'");

    Ok(QRData::new(
        iban.to_string(),
        recipient_address,
        sender_address,
        if amount.is_empty() {
            None
        } else {
            Some(amount.to_string())
        },
        currency.to_string(),
        reference_type.to_string(),
        if reference.is_empty() {
            None
        } else {
            Some(reference.to_string())
        },
        if message.is_empty() {
            None
        } else {
            Some(message.to_string())
        },
    ))
}
