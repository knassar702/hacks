FROM rust:1.31-slim AS build-env
WORKDIR /anti-burl
COPY . .
RUN cargo build
FROM debian:stable-slim
COPY --from=build-env /anti-burl/target/release/anti-burl /usr/bin/anti-burl
ENTRYPOINT ["anti-burl"]
