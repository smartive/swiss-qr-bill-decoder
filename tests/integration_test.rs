use assert_cmd::Command;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn version_long_output() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("{PKG_NAME} {PKG_VERSION}")));
}

#[test]
fn version_short_output() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("{PKG_NAME} {PKG_VERSION}")));
}

#[test]
fn minimal_png() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/minimal.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH4289144165265158476\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"A\",\"address_line_1\":\" \",\"address_line_2\":\"8000 Zürich\",\"country\":\"CH\"},\"sender_address\":null,\"amount\":null,\"currency\":\"CHF\",\"reference_type\":\"NON\",\"reference\":null,\"message\":null}]");
}

#[test]
fn minimal_pdf() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/minimal.pdf")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH4289144165265158476\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"A\",\"address_line_1\":\" \",\"address_line_2\":\"8000 Zürich\",\"country\":\"CH\"},\"sender_address\":null,\"amount\":null,\"currency\":\"CHF\",\"reference_type\":\"NON\",\"reference\":null,\"message\":null}]");
}

#[test]
fn full_png() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/full.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH3389144927977473182\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Test Recipient AG\",\"address_line_1\":\"Teststreet 42a\",\"address_line_2\":\"9000 Zürich\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sender AG\",\"address_line_1\":\"Senderstreet 99C\",\"address_line_2\":\"1234 Sendertown\",\"country\":\"AT\"},\"amount\":\"1337.42\",\"currency\":\"EUR\",\"reference_type\":\"SCOR\",\"reference\":\"RF541234\",\"message\":\"This is a test Message\"}]");
}

#[test]
fn full_pdf() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/full.pdf")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH3389144927977473182\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Test Recipient AG\",\"address_line_1\":\"Teststreet 42a\",\"address_line_2\":\"9000 Zürich\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sender AG\",\"address_line_1\":\"Senderstreet 99C\",\"address_line_2\":\"1234 Sendertown\",\"country\":\"AT\"},\"amount\":\"1337.42\",\"currency\":\"EUR\",\"reference_type\":\"SCOR\",\"reference\":\"RF541234\",\"message\":\"This is a test Message\"}]");
}

#[test]
fn rotated_png() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/rotated.pdf")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH3389144927977473182\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Test Recipient AG\",\"address_line_1\":\"Teststreet 42a\",\"address_line_2\":\"9000 Zürich\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sender AG\",\"address_line_1\":\"Senderstreet 99C\",\"address_line_2\":\"1234 Sendertown\",\"country\":\"AT\"},\"amount\":\"1337.42\",\"currency\":\"EUR\",\"reference_type\":\"SCOR\",\"reference\":\"RF541234\",\"message\":\"This is a test Message\"}]");
}

#[test]
fn rotated_pdf() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/rotated.pdf")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH3389144927977473182\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Test Recipient AG\",\"address_line_1\":\"Teststreet 42a\",\"address_line_2\":\"9000 Zürich\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sender AG\",\"address_line_1\":\"Senderstreet 99C\",\"address_line_2\":\"1234 Sendertown\",\"country\":\"AT\"},\"amount\":\"1337.42\",\"currency\":\"EUR\",\"reference_type\":\"SCOR\",\"reference\":\"RF541234\",\"message\":\"This is a test Message\"}]");
}

#[test]
fn double_png() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/double.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH3389144927977473182\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Test Recipient AG\",\"address_line_1\":\"Teststreet 42a\",\"address_line_2\":\"9000 Zürich\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sender AG\",\"address_line_1\":\"Senderstreet 99C\",\"address_line_2\":\"1234 Sendertown\",\"country\":\"AT\"},\"amount\":\"1337.42\",\"currency\":\"EUR\",\"reference_type\":\"SCOR\",\"reference\":\"RF541234\",\"message\":\"This is a test Message\"},{\"iban\":\"CH4289144165265158476\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"A\",\"address_line_1\":\" \",\"address_line_2\":\"8000 Zürich\",\"country\":\"CH\"},\"sender_address\":null,\"amount\":null,\"currency\":\"CHF\",\"reference_type\":\"NON\",\"reference\":null,\"message\":null}]");
}

#[test]
fn double_pdf() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/double.pdf")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH3389144927977473182\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Test Recipient AG\",\"address_line_1\":\"Teststreet 42a\",\"address_line_2\":\"9000 Zürich\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sender AG\",\"address_line_1\":\"Senderstreet 99C\",\"address_line_2\":\"1234 Sendertown\",\"country\":\"AT\"},\"amount\":\"1337.42\",\"currency\":\"EUR\",\"reference_type\":\"SCOR\",\"reference\":\"RF541234\",\"message\":\"This is a test Message\"},{\"iban\":\"CH4289144165265158476\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"A\",\"address_line_1\":\" \",\"address_line_2\":\"8000 Zürich\",\"country\":\"CH\"},\"sender_address\":null,\"amount\":null,\"currency\":\"CHF\",\"reference_type\":\"NON\",\"reference\":null,\"message\":null}]");
}
#[test]
fn none_png() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/none.pdf")
        .assert()
        .success()
        .stdout("[]");
}

#[test]
fn none_pdf() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/none.pdf")
        .assert()
        .success()
        .stdout("[]");
}

