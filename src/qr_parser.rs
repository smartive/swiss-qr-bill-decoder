use crate::models::address::Address;
use crate::models::qr_data::QRData;
use std::str::Lines;

pub fn get_qr_code_data(text: &String) -> Result<QRData, String> {
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
        Some(iban) if iban.is_empty() => return Err("Missing IBAN".to_string()),
        Some(iban) if iban.starts_with("CH") || iban.starts_with("LI") => iban.to_string(),
        _ => return Err("Only CH and LI IBANs are supported".to_string()),
    };

    let address_type = match lines.next() {
        Some(address_type) if address_type.is_empty() => {
            return Err("Recipient address type is empty".to_string())
        }
        Some(address_type) => address_type,
        _ => return Err("Missing recipient address type".to_string()),
    };

    let recipient_address = to_address(&mut lines, address_type)?;

    skip_lines(&mut lines, 7);

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
        Some(to_address(&mut lines, address_type.unwrap())?)
    } else {
        skip_lines(&mut lines, 6);
        None
    };

    let reference_type = match lines.next() {
        Some(reference_type) if reference_type.is_empty() => {
            return Err("Missing reference type".to_string())
        }
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
        Some(reference) if reference.is_empty() => None,
        Some(reference) => Some(reference.trim().to_string()),
        _ => return Err("Missing reference".to_string()),
    };

    let message = match lines.next() {
        Some(message) if message.is_empty() => None,
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
    Address::new(
        Some(address_type),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
    )
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
                Some("S"),
                Some("Max Muster & Söhne"),
                Some("Musterstrasse"),
                Some("123"),
                Some("8000"),
                Some("Seldwyla"),
                Some("CH"),
            )
            .unwrap(),
            Some(
                Address::new(
                    Some("S"),
                    Some("Simon Muster"),
                    Some("Musterstrasse"),
                    Some("1"),
                    Some("8000"),
                    Some("Seldwyla"),
                    Some("CH"),
                )
                .unwrap(),
            ),
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
