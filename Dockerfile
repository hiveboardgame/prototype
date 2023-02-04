FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim AS hive-base

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev postgresql libpq-dev brotli build-essential git
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install wasm-bindgen-cli 
RUN cargo install wasm-opt
RUN cargo install trunk 
ENV RUST_LOG=info

FROM hive-base AS hive-app

WORKDIR /hive
COPY . .
RUN cargo clean
RUN cd backend; cargo build --release;
RUN cd frontend; trunk build --release --filehash=false; cp -rv dist/* ../backend/dist;
CMD cd backend; cargo run --release --bin backend
