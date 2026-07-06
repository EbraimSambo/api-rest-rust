FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y libpq-dev pkg-config && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api-rest-rust .
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml .

EXPOSE 8080

CMD ["./api-rest-rust"]
