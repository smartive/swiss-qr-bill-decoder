use swiss_qr_bill_decoder_lib::get_qr_bill_data;
use swiss_qr_bill_decoder_lib::models::address::Address;
use swiss_qr_bill_decoder_lib::models::qr_data::QRData;


#[test]
fn minimal_png() {
    let actual = get_qr_bill_data("tests/data/minimal.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH4289144165265158476".to_string(),
        Address::new(
            "S".to_string(),
            "A".to_string(),
            " ".to_string(),
            "8000 Zürich".to_string(),
            "CH".to_string(),
        ),
        None,
        None,
        "CHF".to_string(),
        "NON".to_string(),
        None,
        None,
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn minimal_pdf() {
    let actual = get_qr_bill_data("tests/data/minimal.pdf".to_string(), true);

    let expected = vec![QRData::new(
        "CH4289144165265158476".to_string(),
        Address::new(
            "S".to_string(),
            "A".to_string(),
            " ".to_string(),
            "8000 Zürich".to_string(),
            "CH".to_string(),
        ),
        None,
        None,
        "CHF".to_string(),
        "NON".to_string(),
        None,
        None,
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn full_png() {
    let actual = get_qr_bill_data("tests/data/full.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH3389144927977473182".to_string(),
        Address::new(
            "S".to_string(),
            "Test Recipient AG".to_string(),
            "Teststreet 42a".to_string(),
            "9000 Zürich".to_string(),
            "CH".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sender AG".to_string(),
            "Senderstreet 99C".to_string(),
            "1234 Sendertown".to_string(),
            "AT".to_string(),
        )),
        Some("1337.42".to_string()),
        "EUR".to_string(),
        "SCOR".to_string(),
        Some("RF541234".to_string()),
        Some("This is a test Message".to_string()),
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn full_pdf() {
    let actual = get_qr_bill_data("tests/data/full.pdf".to_string(), true);

    let expected = vec![QRData::new(
        "CH3389144927977473182".to_string(),
        Address::new(
            "S".to_string(),
            "Test Recipient AG".to_string(),
            "Teststreet 42a".to_string(),
            "9000 Zürich".to_string(),
            "CH".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sender AG".to_string(),
            "Senderstreet 99C".to_string(),
            "1234 Sendertown".to_string(),
            "AT".to_string(),
        )),
        Some("1337.42".to_string()),
        "EUR".to_string(),
        "SCOR".to_string(),
        Some("RF541234".to_string()),
        Some("This is a test Message".to_string()),
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn rotated_png() {
    let actual = get_qr_bill_data("tests/data/rotated.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH3389144927977473182".to_string(),
        Address::new(
            "S".to_string(),
            "Test Recipient AG".to_string(),
            "Teststreet 42a".to_string(),
            "9000 Zürich".to_string(),
            "CH".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sender AG".to_string(),
            "Senderstreet 99C".to_string(),
            "1234 Sendertown".to_string(),
            "AT".to_string(),
        )),
        Some("1337.42".to_string()),
        "EUR".to_string(),
        "SCOR".to_string(),
        Some("RF541234".to_string()),
        Some("This is a test Message".to_string()),
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn rotated_pdf() {
    let actual = get_qr_bill_data("tests/data/rotated.pdf".to_string(), true);

    let expected = vec![QRData::new(
        "CH3389144927977473182".to_string(),
        Address::new(
            "S".to_string(),
            "Test Recipient AG".to_string(),
            "Teststreet 42a".to_string(),
            "9000 Zürich".to_string(),
            "CH".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sender AG".to_string(),
            "Senderstreet 99C".to_string(),
            "1234 Sendertown".to_string(),
            "AT".to_string(),
        )),
        Some("1337.42".to_string()),
        "EUR".to_string(),
        "SCOR".to_string(),
        Some("RF541234".to_string()),
        Some("This is a test Message".to_string()),
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn double_png() {
    let actual = get_qr_bill_data("tests/data/double.png".to_string(), true);

    let expected = vec![
        QRData::new(
            "CH3389144927977473182".to_string(),
            Address::new(
                "S".to_string(),
                "Test Recipient AG".to_string(),
                "Teststreet 42a".to_string(),
                "9000 Zürich".to_string(),
                "CH".to_string(),
            ),
            Some(Address::new(
                "S".to_string(),
                "Sender AG".to_string(),
                "Senderstreet 99C".to_string(),
                "1234 Sendertown".to_string(),
                "AT".to_string(),
            )),
            Some("1337.42".to_string()),
            "EUR".to_string(),
            "SCOR".to_string(),
            Some("RF541234".to_string()),
            Some("This is a test Message".to_string()),
        ),
        QRData::new(
            "CH4289144165265158476".to_string(),
            Address::new(
                "S".to_string(),
                "A".to_string(),
                " ".to_string(),
                "8000 Zürich".to_string(),
                "CH".to_string(),
            ),
            None,
            None,
            "CHF".to_string(),
            "NON".to_string(),
            None,
            None,
        ),
    ];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}


#[test]
fn double_pdf() {
    let actual = get_qr_bill_data("tests/data/double.pdf".to_string(), true);

    let expected = vec![
        QRData::new(
            "CH3389144927977473182".to_string(),
            Address::new(
                "S".to_string(),
                "Test Recipient AG".to_string(),
                "Teststreet 42a".to_string(),
                "9000 Zürich".to_string(),
                "CH".to_string(),
            ),
            Some(Address::new(
                "S".to_string(),
                "Sender AG".to_string(),
                "Senderstreet 99C".to_string(),
                "1234 Sendertown".to_string(),
                "AT".to_string(),
            )),
            Some("1337.42".to_string()),
            "EUR".to_string(),
            "SCOR".to_string(),
            Some("RF541234".to_string()),
            Some("This is a test Message".to_string()),
        ),
        QRData::new(
            "CH4289144165265158476".to_string(),
            Address::new(
                "S".to_string(),
                "A".to_string(),
                " ".to_string(),
                "8000 Zürich".to_string(),
                "CH".to_string(),
            ),
            None,
            None,
            "CHF".to_string(),
            "NON".to_string(),
            None,
            None,
        ),
    ];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn none_png() {
    let actual = get_qr_bill_data("tests/data/none.png".to_string(), true);

    assert!(actual.is_empty());
}

#[test]
fn none_pdf() {
    let actual = get_qr_bill_data("tests/data/none.pdf".to_string(), true);

    assert!(actual.is_empty());
}