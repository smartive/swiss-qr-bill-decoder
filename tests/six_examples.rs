use swiss_qr_bill_decoder::get_qr_bill_data;
use swiss_qr_bill_decoder::models::address::Address;
use swiss_qr_bill_decoder::models::qr_data::QRData;

#[test]
fn six_example_01() {
    let actual = get_qr_bill_data("tests/data/six_examples/01.png".to_string(), true);

    let expected = vec![QRData::new(
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
        Some("Payment of travel".to_string()),
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn six_example_02() {
    let actual = get_qr_bill_data("tests/data/six_examples/02.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH4431999123000889012".to_string(),
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
        Some("1949.75".to_string()),
        "CHF".to_string(),
        "QRR".to_string(),
        Some("210000000003139471430009017".to_string()),
        Some("Order from 15.10.2020".to_string()),
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn six_example_03() {
    let actual = get_qr_bill_data("tests/data/six_examples/03.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH5204835012345671000".to_string(),
        Address::new(
            "S".to_string(),
            "Sample Foundation".to_string(),
            "PO Box ".to_string(),
            "3001 Bern".to_string(),
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
fn six_example_04() {
    let actual = get_qr_bill_data("tests/data/six_examples/04.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH5800791123000889012".to_string(),
        Address::new(
            "S".to_string(),
            "Muster Krankenkasse".to_string(),
            "Musterstrasse 12".to_string(),
            "8000 Seldwyla".to_string(),
            "CH".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sarah Beispiel".to_string(),
            "Musterstrasse 1".to_string(),
            "8000 Seldwyla".to_string(),
            "CH".to_string(),
        )),
        Some("211.00".to_string()),
        "CHF".to_string(),
        "SCOR".to_string(),
        Some("RF240191230100405JSH0438".to_string()),
        None,
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn six_example_05() {
    let actual = get_qr_bill_data("tests/data/six_examples/05.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH5800791123000889012".to_string(),
        Address::new(
            "S".to_string(),
            "Max Muster & Söhne".to_string(),
            "Musterstrasse 123".to_string(),
            "9490 Vaduz".to_string(),
            "LI".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sarah Beispiel".to_string(),
            "Musterstrasse 1".to_string(),
            "8000 Seldwyla".to_string(),
            "CH".to_string(),
        )),
        Some("199.95".to_string()),
        "CHF".to_string(),
        "SCOR".to_string(),
        Some("RF18539007547034".to_string()),
        None,
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}

#[test]
fn six_example_06() {
    let actual = get_qr_bill_data("tests/data/six_examples/06.png".to_string(), true);

    let expected = vec![QRData::new(
        "CH5800791123000889012".to_string(),
        Address::new(
            "S".to_string(),
            "Max Muster & Söhne".to_string(),
            "Musterstrasse 123".to_string(),
            "8000 Seldwyla".to_string(),
            "CH".to_string(),
        ),
        Some(Address::new(
            "S".to_string(),
            "Sarah Beispiel".to_string(),
            "Musterstrasse 1".to_string(),
            "78462 Konstanz".to_string(),
            "DE".to_string(),
        )),
        Some("199.95".to_string()),
        "CHF".to_string(),
        "SCOR".to_string(),
        Some("RF18539007547034".to_string()),
        None,
    )];

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual[0], expected[0]);
}
