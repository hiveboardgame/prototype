FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . ./

CMD bash -c "cd /app/frontend && trunk serve --address 0.0.0.0 --port 8080"
