use crate::models::address::Address;
use crate::models::qr_data::QRData;
use std::str::Lines;

/// Get the QR code data from a String according to the Swiss QR bill standard
pub fn get_qr_code_data(text: &str) -> Result<QRData, String> {
    let mut lines = text.lines();

    check_line(&mut lines, "SPC", "First line is not 'SPC'")?;
    check_line(&mut lines, "0200", "Only version 0200 is supported")?;
    check_line(&mut lines, "1", "Only coding type 1 (UTF-8) is supported")?;

    let iban = match lines.next() {
        Some("") => return Err("Missing IBAN".to_string()),
        Some(iban) if iban.starts_with("CH") || iban.starts_with("LI") => iban.to_string(),
        _ => return Err("Only CH and LI IBANs are supported".to_string()),
    };

    let address_type = lines.next().ok_or("Missing recipient address type")?;
    let recipient_address = to_address(&mut lines, address_type)?;

    skip_lines(&mut lines, 7);

    let amount = lines.next().filter(|s| !s.is_empty()).map(str::to_string);
    let currency = match lines.next() {
        Some("") => return Err("Missing currency".to_string()),
        Some(currency) if currency == "CHF" || currency == "EUR" => currency.to_string(),
        _ => return Err("Only CHF and EUR currencies are supported".to_string()),
    };

    let address_type = lines.next().filter(|s| !s.is_empty());
    let sender_address = if let Some(address_type) = address_type {
        Some(to_address(&mut lines, address_type)?)
    } else {
        skip_lines(&mut lines, 6);
        None
    };

    let reference_type = match lines.next() {
        Some("") => return Err("Missing reference type".to_string()),
        Some(reference_type) if ["NON", "QRR", "SCOR"].contains(&reference_type) => {
            reference_type.to_string()
        }
        _ => return Err("Only reference types NON, QRR, and SCOR are supported".to_string()),
    };

    let reference = match lines.next() {
        Some(reference)
            if reference.is_empty() && ["QRR", "SCOR"].contains(&reference_type.as_str()) =>
        {
            return Err("Reference is empty".to_string());
        }
        Some("") => None,
        Some(reference) => Some(reference.trim().to_string()),
        _ => return Err("Missing reference".to_string()),
    };

    let message = lines.next().filter(|s| !s.is_empty()).map(str::to_string);
    check_line(&mut lines, "EPD", "Missing trailing 'EPD'")?;

    let billing_info = lines.next().filter(|s| !s.is_empty()).map(str::to_string);

    Ok(QRData::new(
        iban,
        recipient_address,
        sender_address,
        amount,
        currency,
        reference_type,
        reference,
        message,
        billing_info,
    ))
}

fn check_line(lines: &mut Lines, expected: &str, error_msg: &str) -> Result<(), String> {
    if lines.next() != Some(expected) {
        return Err(error_msg.to_string());
    }
    Ok(())
}

fn skip_lines(lines: &mut Lines, skip_count: i32) {
    for _ in 0..skip_count {
        let _ = lines.next();
    }
}

fn to_address(lines: &mut Lines, address_type: &str) -> Result<Address, String> {
    let address_type = match address_type {
        "" => return Err("Address type is empty".to_string()),
        address_type if !["K", "S"].contains(&address_type) => {
            return Err("Only address types K and S are supported".to_string())
        }
        address_type => address_type.to_string(),
    };

    let name = lines.next().ok_or("Missing name".to_string())?.to_string();
    let street_or_address_line_1 = lines
        .next()
        .ok_or("Missing street or address line 1".to_string())?
        .to_string();
    let building_number_or_address_line_2 = lines
        .next()
        .ok_or("Missing building number or address line 2".to_string())?
        .to_string();
    let postal_code = lines
        .next()
        .ok_or("Missing postal code".to_string())?
        .to_string();
    let town = lines.next().ok_or("Missing town".to_string())?.to_string();
    let country = lines
        .next()
        .ok_or("Missing country".to_string())?
        .to_string();

    let address_line_1 = if address_type == "K" {
        street_or_address_line_1.clone()
    } else {
        format!("{street_or_address_line_1} {building_number_or_address_line_2}")
    };

    let address_line_2 = if address_type == "K" {
        building_number_or_address_line_2.clone()
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
        //S1/10/1234/11/201021/30/102673386/32/7.7/40/0:30
        "#};

        let qr_code_data = get_qr_code_data(MY_CONST)?;

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
            Some("//S1/10/1234/11/201021/30/102673386/32/7.7/40/0:30".to_string()),
        );

        assert_eq!(expected, qr_code_data);

        Ok(())
    }
}
