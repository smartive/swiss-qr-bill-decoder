FROM rust:1.93-alpine@sha256:69d7b9d9aeaf108a1419d9a7fcf7860dcc043e9dbd1ab7ce88e44228774d99e9 as build

RUN apk add --update --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo install --path cli

FROM node:24-alpine@sha256:cd6fb7efa6490f039f3471a189214d5f548c11df1ff9e5b181aa49e22c14383e
COPY --from=build /usr/local/cargo/bin/swiss-qr-bill-decoder /usr/local/bin/swiss-qr-bill-decoder

RUN chmod +x /usr/local/bin/swiss-qr-bill-decoder
CMD ["swiss-qr-bill-decoder", "--help"]