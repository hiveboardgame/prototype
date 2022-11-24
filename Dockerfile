FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim AS base

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev postgresql libpq-dev brotli build-essential
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install trunk wasm-bindgen-cli cargo-watch wasm-opt
RUN cargo install diesel_cli --no-default-features --features postgres
ENV RUST_LOG=info

FROM base AS hive

WORKDIR /app
COPY backend/ /app/backend
COPY frontend/ /app/frontend
COPY engine/ /app/engine
RUN cd /app/backend; cargo build --release;
RUN cd /app/frontend; trunk build --release --filehash=false; cp -rv dist/* /app/backend/dist/;

CMD cd /app/backend; cargo run --release
