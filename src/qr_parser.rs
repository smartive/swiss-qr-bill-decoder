use crate::models::address::Address;
use crate::models::qr_data::QRData;

pub fn get_qr_code_data(text: &String) -> Result<QRData, String> {
    let mut lines = text.lines();

    lines.next().ok_or("Missing first line".to_string())?;
    
    match lines.next() {
        Some("SPC") => {}
        _ => return Err("First line is not 'SPC'".to_string()),
    }

    match lines.next() {
        Some("0200") => {}
        _ => return Err("Only version 0200 is supported".to_string()),
    }

    match lines.next() {
        Some("1") => {}
        _ => return Err("Only coding type 1 (UTF-8) is supported".to_string()),
    }

    let iban = match lines.next() {
        Some(iban) if iban.is_empty() => return Err("Missing IBAN".to_string()),
        Some(iban) if iban.starts_with("CH") || iban.starts_with("LI") => iban.to_string(),
        _ => return Err("Only CH and LI IBANs are supported".to_string()),
    };

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

    let amount = match lines.next() {
        Some(amount) if amount.is_empty() => None,
        Some(amount) => Some(amount.trim().to_string()),
        _ => return Err("Missing amount".to_string()),
    };

    let currency = match lines.next() {
        Some(currency) if currency.is_empty() => return Err("Missing currency".to_string()),
        Some(currency) if currency.eq("CHF") || currency.eq("EUR") => currency.to_string(),
        _ => return Err("Only CHF and EUR currencies are supported".to_string()),
    };

    let address_type = match lines.next() {
        Some(address_type) if address_type.is_empty() => None,
        Some(address_type) => Some(address_type),
        _ => return Err("Missing address type".to_string()),
    };

    let sender_address = if address_type.is_some() {
        Some(Address::new(
            Some(address_type.unwrap()),
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

    let reference_type = match lines.next() {
        Some(reference_type) if reference_type.is_empty() => return Err("Missing reference type".to_string()),
        Some(reference_type) if reference_type.eq("NON") || reference_type.eq("QRR") || reference_type.eq("SCOR") => reference_type.to_string(),
        _ => return Err("Only reference types NON, QRR and SCOR are supported".to_string()),
    };

    let reference = match lines.next() {
        Some(reference) if reference.is_empty() && (reference_type.eq("QRR") || reference_type.eq("SCOR")) => return Err("Reference is empty".to_string()),
        Some(reference) if reference.is_empty() => None,
        Some(reference) => Some(reference.trim().to_string()),
        _ => return Err("Missing reference".to_string()),
    };

    let message = match lines.next() {
        Some(message) if message.is_empty() => None,
        Some(message) => Some(message.trim().to_string()),
        _ => return Err("Missing message".to_string()),
    };

    match lines.next() {
        Some("EPD") => {}
        _ => return Err("Missing trailing 'EPD'".to_string()),
    }

    Ok(QRData::new(
        iban,
        recipient_address,
        sender_address,
        amount,
        currency,
        reference_type,
        reference,
        message,
    ))
}
