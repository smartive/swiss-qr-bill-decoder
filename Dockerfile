FROM rust:1.94-alpine@sha256:7f752ee8ea5deb9f4863d8c3f228a216a6466619882f09a44b9eda9617dc7770 as build

RUN apk add --update --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo install --path cli

FROM node:24-alpine@sha256:cd6fb7efa6490f039f3471a189214d5f548c11df1ff9e5b181aa49e22c14383e
COPY --from=build /usr/local/cargo/bin/swiss-qr-bill-decoder /usr/local/bin/swiss-qr-bill-decoder

RUN chmod +x /usr/local/bin/swiss-qr-bill-decoder
CMD ["swiss-qr-bill-decoder", "--help"]