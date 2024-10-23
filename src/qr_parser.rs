use crate::models::address::Address;
use crate::models::qr_data::QRData;
use std::str::Lines;

/// Get the QR code data from a String according to the Swiss QR bill standard
pub fn get_qr_code_data(text: &str) -> Result<QRData, String> {
    let mut lines = text.lines();

    if lines.next() != Some("SPC") {
        return Err("First line is not 'SPC'".to_string());
    }

    if lines.next() != Some("0200") {
        return Err("Only version 0200 is supported".to_string());
    }

    if lines.next() != Some("1") {
        return Err("Only coding type 1 (UTF-8) is supported".to_string());
    }

    let iban = match lines.next() {
        Some("") => return Err("Missing IBAN".to_string()),
        Some(iban) if iban.starts_with("CH") || iban.starts_with("LI") => iban.to_string(),
        _ => return Err("Only CH and LI IBANs are supported".to_string()),
    };

    let address_type = match lines.next() {
        Some("") => return Err("Recipient address type is empty".to_string()),
        Some(address_type) => address_type,
        _ => return Err("Missing recipient address type".to_string()),
    };

    let recipient_address = to_address(&mut lines, address_type)?;

    skip_lines(&mut lines, 7);

    let amount = match lines.next() {
        Some("") => None,
        Some(amount) => Some(amount.trim().to_string()),
        _ => return Err("Missing amount".to_string()),
    };

    let currency = match lines.next() {
        Some("") => return Err("Missing currency".to_string()),
        Some(currency) if currency.eq("CHF") || currency.eq("EUR") => currency.to_string(),
        _ => return Err("Only CHF and EUR currencies are supported".to_string()),
    };

    let address_type = match lines.next() {
        Some("") => None,
        Some(address_type) => Some(address_type),
        _ => return Err("Missing address type".to_string()),
    };

    let sender_address = if address_type.is_some() {
        Some(to_address(&mut lines, address_type.unwrap())?)
    } else {
        skip_lines(&mut lines, 6);
        None
    };

    let reference_type = match lines.next() {
        Some("") => return Err("Missing reference type".to_string()),
        Some(reference_type)
            if reference_type.eq("NON")
                || reference_type.eq("QRR")
                || reference_type.eq("SCOR") =>
        {
            reference_type.to_string()
        }
        _ => return Err("Only reference types NON, QRR and SCOR are supported".to_string()),
    };

    let reference = match lines.next() {
        Some(reference)
            if reference.is_empty() && (reference_type.eq("QRR") || reference_type.eq("SCOR")) =>
        {
            return Err("Reference is empty".to_string())
        }
        Some("") => None,
        Some(reference) => Some(reference.trim().to_string()),
        _ => return Err("Missing reference".to_string()),
    };

    let message = match lines.next() {
        Some("") => None,
        Some(message) => Some(message.trim().to_string()),
        _ => return Err("Missing message".to_string()),
    };

    if lines.next() != Some("EPD") {
        return Err("Missing trailing 'EPD'".to_string());
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

fn skip_lines(lines: &mut Lines, skip_lines: i32) {
    for _ in 0..skip_lines {
        let _ = lines.next();
    }
}

fn to_address(lines: &mut Lines, address_type: &str) -> Result<Address, String> {
    let address_type = match address_type {
        "" => return Err("Address type is empty".to_string()),
        address_type if !address_type.eq("K") && !address_type.eq("S") => {
            return Err("Only address types K and S are supported".to_string())
        }
        address_type => address_type.to_string(),
    };

    let name = match lines.next() {
        None => return Err("Missing name".to_string()),
        Some("") => return Err("Recipient name is empty".to_string()),
        Some(name) => name.to_string(),
    };

    let street_or_address_line_1 = match lines.next() {
        None => return Err("Missing street or address line 1".to_string()),
        Some(street_or_address_line_1) => street_or_address_line_1.to_string(),
    };

    let building_number_or_address_line_2 = match lines.next() {
        None => return Err("Missing building number or address line 2".to_string()),
        Some(building_number_or_address_line_2) => building_number_or_address_line_2.to_string(),
    };

    let postal_code = match lines.next() {
        None => return Err("Missing postal code".to_string()),
        Some(postal_code) => postal_code.to_string(),
    };

    let town = match lines.next() {
        None => return Err("Missing town".to_string()),
        Some(town) => town.to_string(),
    };

    let country = match lines.next() {
        None => return Err("Missing country".to_string()),
        Some(country) => country.to_string(),
    };

    let address_line_1 = if address_type.eq("K") {
        street_or_address_line_1.to_string()
    } else {
        format!("{street_or_address_line_1} {building_number_or_address_line_2}")
    };

    let address_line_2 = if address_type.eq("K") {
        building_number_or_address_line_2.to_string()
    } else {
        format!("{postal_code} {town}")
    };

    Ok(Address::new(
        address_type,
        name,
        address_line_1,
        address_line_2,
        country,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_get_qr_code_data() -> Result<(), String> {
        const MY_CONST: &str = indoc! {r#"
        SPC
        0200
        1
        CH6431961000004421557
        S
        Max Muster & Söhne
        Musterstrasse
        123
        8000
        Seldwyla
        CH







        50.00
        CHF
        S
        Simon Muster
        Musterstrasse
        1
        8000
        Seldwyla
        CH
        QRR
        000008207791225857421286694
        Bezahlung der Reise
        EPD


        "#};

        let qr_code_data = get_qr_code_data(&MY_CONST.to_string())?;

        let expected = QRData::new(
            "CH6431961000004421557".to_string(),
            Address::new(
                "S".to_string(),
                "Max Muster & Söhne".to_string(),
                "Musterstrasse 123".to_string(),
                "8000 Seldwyla".to_string(),
                "CH".to_string(),
            ),
            Some(Address::new(
                "S".to_string(),
                "Simon Muster".to_string(),
                "Musterstrasse 1".to_string(),
                "8000 Seldwyla".to_string(),
                "CH".to_string(),
            )),
            Some("50.00".to_string()),
            "CHF".to_string(),
            "QRR".to_string(),
            Some("000008207791225857421286694".to_string()),
            Some("Bezahlung der Reise".to_string()),
        );

        assert_eq!(expected, qr_code_data);

        Ok(())
    }
}
