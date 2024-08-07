# Swiss QR bill decoder
This project contains a tool to decode the Swiss QR bill. It reads the QR code from an image or a pdf and decodes the data json based on the [Swiss QR bill specification](https://www.six-group.com/en/products-services/banking-services/payment-standardization/standards/qr-bill.html).

It contains a lib crate that can be used as a library and a bin crate that can be used as a command line tool.


> [!WARNING]
> Users of this project do so at their own risk. There are no guarantees of any kind regarding the reliability, functionality, or security of this tool. The authors are not responsible for any damage or losses incurred from using this software.

## Usage

To run the tool in debug mode, use the following command:
```shell
cargo run -- <path_to_image>
```
For additional options, run:
```shell
cargo run -- --help
```

To build a release version, use the following command:
```shell
cargo build --release
```
The binary will be located at `target/release/swiss-qr-bill-decoder` and can be run as follows:
```shell
./target/release/swiss-qr-bill-decoder <path_to_image>
```

## Run tests
```shell
cargo test
```