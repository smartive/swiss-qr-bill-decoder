FROM rust:1.100-alpine@sha256:de1e0058543a81ed7b046629167db587c67a94bdf37cce14eb27f0ec83baaffb AS build

RUN apk add --update --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM alpine:3.24.4@sha256:f59abe656b692d901e184be3445e084786602108599d2a51d229c0f5a48c71bf
COPY --from=build /usr/local/cargo/bin/swiss-qr-bill-decoder /usr/local/bin/swiss-qr-bill-decoder

RUN chmod +x /usr/local/bin/swiss-qr-bill-decoder
CMD ["swiss-qr-bill-decoder", "--help"]