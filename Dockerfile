FROM rust:1.89-alpine as build

RUN apk add --update --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo install --path cli

FROM node:22-alpine
COPY --from=build /usr/local/cargo/bin/swiss-qr-bill-decoder /usr/local/bin/swiss-qr-bill-decoder

RUN chmod +x /usr/local/bin/swiss-qr-bill-decoder
CMD ["swiss-qr-bill-decoder", "--help"]