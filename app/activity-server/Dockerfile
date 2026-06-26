FROM rust:1-alpine AS build

WORKDIR /

COPY Cargo.toml Cargo.lock ./
COPY app/activity-server ./app/activity-server
COPY service/discord-auth ./service/discord-auth

RUN cargo build \
    --release \
    --manifest-path app/activity-server/Cargo.toml

FROM ubuntu:latest

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

COPY --from=build /target/release/activity-server /app

EXPOSE 8510

ENTRYPOINT ["/app"]

# docker build --push --progress=plain -t parkjungin/discord-activity-server:1.0.0 -f dev -f app/activity-server/Dockerfile .