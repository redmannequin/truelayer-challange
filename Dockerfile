FROM rust:1.55-slim-buster AS builder
RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config ca-certificates && update-ca-certificates
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin pokedex

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config ca-certificates && update-ca-certificates
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/pokedex* /app/
COPY ./pokedex/Rocket.toml .
CMD ["./pokedex"]