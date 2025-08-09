FROM rust:1.89 AS builder
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target true
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked
RUN rm -rf src

COPY src ./src
COPY assets ./assets
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --locked && \
    ls -la target/release && \
    strip target/release/tech_icons || true

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/tech_icons /app/tech_icons
COPY --from=builder /app/assets /app/assets
RUN useradd -r -s /usr/sbin/nologin appuser && chown -R appuser:appuser /app
USER appuser
ENV RUST_LOG=info PORT=3000
EXPOSE 3000
CMD ["/app/tech_icons"]
