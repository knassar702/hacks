# Build
FROM rust:1.31-slim AS build-env
WORKDIR /get-title
COPY . .
RUN cargo build --release

# Release
FROM debian:stable-slim
COPY --from=build-env /get-title/target/release/get-title /usr/bin/get-title
ENTRYPOINT ["get-title"]
