# syntax=docker/dockerfile:1.7

# Keep the Rust toolchain out of the runtime image.  The manifest-only build
# makes dependency compilation cacheable when application sources change.
FROM rust:1.97.1-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    mkdir src \
    && printf 'fn main() {}\n' > src/main.rs \
    && cargo build --locked --release --bin openapi \
    && rm -f target/release/openapi target/release/deps/openapi-*

COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --locked --release --bin openapi

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 app \
    && useradd --uid 10001 --gid app --create-home --shell /usr/sbin/nologin app

WORKDIR /app
COPY --from=builder --chown=app:app /app/target/release/openapi ./openapi
COPY --chown=app:app static ./static

ENV RUST_LOG=error,openapi=info
USER app

EXPOSE 8000
CMD ["./openapi"]
