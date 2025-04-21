# Build stage
FROM rust:1.86-slim AS builder
WORKDIR /usr/src/ws-rs

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// placeholder" > src/lib.rs
RUN cargo fetch

COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /usr/src/ws-rs/target/release/ws-rs .

EXPOSE 8000
CMD ["./ws-rs"]