FROM rust:1.97-alpine@sha256:3c38f3f82c2f3d73da3b38e18d279393a04cb43ddded0e35088a8c3324d40900 AS build

RUN apk add --update --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM alpine:3.24.1@sha256:bec4ccd3817e7c824eb0388971a0b83fab111d586285511ba0266b77e8dc65a9
COPY --from=build /usr/local/cargo/bin/swiss-qr-bill-decoder /usr/local/bin/swiss-qr-bill-decoder

RUN chmod +x /usr/local/bin/swiss-qr-bill-decoder
CMD ["swiss-qr-bill-decoder", "--help"]