FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev
RUN cargo install trunk wasm-bindgen-cli cargo-watch
RUN rustup target add wasm32-unknown-unknown

ENV RUST_LOG=info

WORKDIR /app
COPY . ./

RUN bash -c "cd /app/backend && cargo build"
RUN bash -c "cd /app/frontend && trunk build"

CMD /app/run.sh
