FROM rust:1.71  as builder

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    ca-certificates


RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /build

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:bullseye-slim AS publish

RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-gnu/release/whatstheweather .

ENTRYPOINT ["/app/whatstheweather"]
