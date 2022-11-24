FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev postgresql libpq-dev brotli build-essential
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli cargo-watch wasm-opt
RUN cargo install diesel_cli --no-default-features --features postgres
ENV RUST_LOG=info

WORKDIR /app
COPY . ./

RUN bash -c "cd /app/backend && cargo build --release"
RUN bash -c "cd /app/frontend && trunk build --release"

CMD /app/run.sh
