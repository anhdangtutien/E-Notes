# Multi-stage Dockerfile for E-Notes (100% Rust Core + Web Static)
# Builder stage
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libsqlite3-dev && rm -rf /var/lib/apt/lists/*

# Copy source code and static assets
COPY Cargo.toml ./
COPY src ./src
COPY static ./static

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates libsqlite3-0 && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /data /app

# Copy compiled binary and static assets from builder
COPY --from=builder /usr/src/app/target/release/enotes /app/enotes
COPY --from=builder /usr/src/app/static /app/static

# Environment variables
ENV PORT=8080
ENV HOST=0.0.0.0
ENV ENOTES_DB=/app/enotes.db

# Persistent storage volume for SQLite
VOLUME ["/data"]

EXPOSE 8080

CMD ["/app/enotes"]
