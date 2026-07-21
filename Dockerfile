FROM rust:1.90-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && echo 'pub fn placeholder() {}' > src/lib.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY migrations migrations
COPY src src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -m -s /bin/bash appuser

RUN mkdir -p /data && chown appuser:appuser /data

COPY --from=builder /app/target/release/calm-rss /usr/local/bin/calm-rss

USER appuser

ENV DATABASE_URL=/data/calm.db
ENV LISTEN_ADDR=0.0.0.0:3000

EXPOSE 3000

ENTRYPOINT ["calm-rss"]
