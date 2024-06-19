use assert_cmd::Command;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn six_example_01() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/six_examples/01.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH6431961000004421557\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Max Muster & Söhne\",\"address_line_1\":\"Musterstrasse 123\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Simon Muster\",\"address_line_1\":\"Musterstrasse 1\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"amount\":\"50.00\",\"currency\":\"CHF\",\"reference_type\":\"QRR\",\"reference\":\"000008207791225857421286694\",\"message\":\"Payment of travel\"}]");
}

#[test]
fn six_example_02() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/six_examples/01.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH6431961000004421557\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Max Muster & Söhne\",\"address_line_1\":\"Musterstrasse 123\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Simon Muster\",\"address_line_1\":\"Musterstrasse 1\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"amount\":\"50.00\",\"currency\":\"CHF\",\"reference_type\":\"QRR\",\"reference\":\"000008207791225857421286694\",\"message\":\"Payment of travel\"}]");
}

#[test]
fn six_example_03() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/six_examples/03.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH5204835012345671000\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Sample Foundation\",\"address_line_1\":\"PO Box \",\"address_line_2\":\"3001 Bern\",\"country\":\"CH\"},\"sender_address\":null,\"amount\":null,\"currency\":\"CHF\",\"reference_type\":\"NON\",\"reference\":null,\"message\":null}]");
}

#[test]
fn six_example_04() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/six_examples/04.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH5800791123000889012\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Muster Krankenkasse\",\"address_line_1\":\"Musterstrasse 12\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sarah Beispiel\",\"address_line_1\":\"Musterstrasse 1\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"amount\":\"211.00\",\"currency\":\"CHF\",\"reference_type\":\"SCOR\",\"reference\":\"RF240191230100405JSH0438\",\"message\":null}]");
}

#[test]
fn six_example_05() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/six_examples/05.png")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH5800791123000889012\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Max Muster & Söhne\",\"address_line_1\":\"Musterstrasse 123\",\"address_line_2\":\"9490 Vaduz\",\"country\":\"LI\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sarah Beispiel\",\"address_line_1\":\"Musterstrasse 1\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"amount\":\"199.95\",\"currency\":\"CHF\",\"reference_type\":\"SCOR\",\"reference\":\"RF18539007547034\",\"message\":null}]");
}

#[test]
fn six_example_06() {
    Command::cargo_bin(PKG_NAME)
        .unwrap()
        .arg("tests/data/six_examples/06.png")
        .arg("--fail-on-error")
        .assert()
        .success()
        .stdout("[{\"iban\":\"CH5800791123000889012\",\"recipient_address\":{\"address_type\":\"S\",\"name\":\"Max Muster & Söhne\",\"address_line_1\":\"Musterstrasse 123\",\"address_line_2\":\"8000 Seldwyla\",\"country\":\"CH\"},\"sender_address\":{\"address_type\":\"S\",\"name\":\"Sarah Beispiel\",\"address_line_1\":\"Musterstrasse 1\",\"address_line_2\":\"78462 Konstanz\",\"country\":\"DE\"},\"amount\":\"199.95\",\"currency\":\"CHF\",\"reference_type\":\"SCOR\",\"reference\":\"RF18539007547034\",\"message\":null}]");
}