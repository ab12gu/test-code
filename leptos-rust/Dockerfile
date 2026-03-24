# Faster build than rust:nightly-bullseye which is full debian
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /usr/src/app

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    perl \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest and create dummy src to fetch deps
# This is to avoid downloading all the dependencies every time we change the source code
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy full source
COPY . .

# Build the named binary explicitly
RUN cargo build --release --bin leptos-test

# Build the wasm target
# RUN rustup target add wasm32-unknown-unknown

# Final stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/leptos-test .

CMD ["./leptos-test"]


