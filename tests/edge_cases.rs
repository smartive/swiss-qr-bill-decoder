use swiss_qr_bill_decoder::get_qr_bill_data;
use swiss_qr_bill_decoder::models::address::Address;
use swiss_qr_bill_decoder::models::qr_data::QRData;

#[test]
fn minimal_png() {
    let actual = get_qr_bill_data("tests/data/minimal.png", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_minimal());
}

#[test]
fn minimal_jpg() {
    let actual = get_qr_bill_data("tests/data/minimal.jpg", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_minimal());
}

#[test]
fn minimal_jpeg() {
    let actual = get_qr_bill_data("tests/data/minimal.jpeg", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_minimal());
}

#[test]
#[ignore]
fn minimal_pdf() {
    let actual = get_qr_bill_data("tests/data/minimal.pdf", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_minimal());
}

#[test]
fn full_png() {
    let actual = get_qr_bill_data("tests/data/full.png", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_full());
}

#[test]
fn full_jpg() {
    let actual = get_qr_bill_data("tests/data/full.jpg", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_full());
}

#[test]
fn full_jpeg() {
    let actual = get_qr_bill_data("tests/data/full.jpeg", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_full());
}

#[test]
#[ignore]
fn full_pdf() {
    let actual = get_qr_bill_data("tests/data/full.pdf", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_full());
}

#[test]
fn rotated_png() {
    let actual = get_qr_bill_data("tests/data/rotated.png", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_rotated());
}

#[test]
fn rotated_jpg() {
    let actual = get_qr_bill_data("tests/data/rotated.jpg", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_rotated());
}

#[test]
fn rotated_jpeg() {
    let actual = get_qr_bill_data("tests/data/rotated.jpeg", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_rotated());
}

#[test]
#[ignore]
fn rotated_pdf() {
    let actual = get_qr_bill_data("tests/data/rotated.pdf", true);

    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], expected_rotated());
}

#[test]
fn double_png() {
    let actual = get_qr_bill_data("tests/data/double.png", true);

    let expected = vec![expected_double_1(), expected_double_2()];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
    assert_eq!(actual[1], expected[1]);
}

#[test]
fn double_jpg() {
    let actual = get_qr_bill_data("tests/data/double.jpg", true);

    let expected = vec![expected_double_1(), expected_double_2()];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
    assert_eq!(actual[1], expected[1]);
}

#[test]
fn double_jpeg() {
    let actual = get_qr_bill_data("tests/data/double.jpeg", true);

    let expected = vec![expected_double_1(), expected_double_2()];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
    assert_eq!(actual[1], expected[1]);
}

#[test]
#[ignore]
fn double_pdf() {
    let actual = get_qr_bill_data("tests/data/double.pdf", true);
    let expected = vec![expected_double_1(), expected_double_2()];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
    assert_eq!(actual[1], expected[1]);
}

#[test]
fn none_png() {
    let actual = get_qr_bill_data("tests/data/none.png", true);

    assert!(actual.is_empty());
}

#[test]
fn none_jpg() {
    let actual = get_qr_bill_data("tests/data/none.jpg", true);

    assert!(actual.is_empty());
}

#[test]
fn none_jpeg() {
    let actual = get_qr_bill_data("tests/data/none.jpeg", true);

    assert!(actual.is_empty());
}

#[test]
#[ignore]
fn none_pdf() {
    let actual = get_qr_bill_data("tests/data/none.pdf", true);

    assert!(actual.is_empty());
}

pub fn expected_minimal() -> QRData {
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
    )
}

pub fn expected_full() -> QRData {
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
    )
}

pub fn expected_rotated() -> QRData {
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
    )
}

pub fn expected_double_1() -> QRData {
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
    )
}

pub fn expected_double_2() -> QRData {
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
    )
}
