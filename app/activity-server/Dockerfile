FROM rust:1-alpine as BUILDER

WORKDIR /

COPY app/activity-server ./app/activity-server
COPY service/discord-auth ./service/discord-auth

RUN cargo build --release

FROM ubuntu:latest

COPY --from=BUILDER /app/target/release/app /app

EXPOSE 8510

ENTRYPOINT ["/app"]

# docker build --push --progress=plain -t parkjungin/discord-activity-server:1.0.0 -f dev -f app/activity-server/Dockerfile .