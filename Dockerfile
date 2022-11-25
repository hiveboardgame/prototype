FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim AS base

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev postgresql libpq-dev brotli build-essential git
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install trunk wasm-bindgen-cli cargo-watch wasm-opt
RUN cargo install diesel_cli --no-default-features --features postgres
ENV RUST_LOG=info

FROM base AS builder

RUN git clone https://github.com/klautcomputing/hive.git
WORKDIR /hive
RUN cd backend; cargo build --release;
RUN cd frontend; trunk build --release --filehash=false; cp -rv dist/* ../backend/dist;
CMD cd backend; cargo run --release --bin backend
