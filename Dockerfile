FROM rust:1.55-slim-buster AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/pokedex* /app/
COPY ./Rocket.toml .
CMD ["./pokedex"]