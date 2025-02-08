FROM rust:latest AS build

WORKDIR /app

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

FROM rust:slim

WORKDIR /app

COPY --from=build /app/target/release/telnet_pit .

CMD ["./telnet_pit"]