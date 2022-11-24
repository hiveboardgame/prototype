#!/usr/bin/env bash

children=()

_term() {
    echo "Caught SIGTERM"
    for child in "${children[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done 
}

_int() {
    echo "Caught SIGINT"
    for child in "${children[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done 
}

trap _term SIGTERM
trap _int SIGINT

pushd frontend;
trunk build --release --filehash=false
# brotli -v dist/*
# mv -v dist/frontend_bg.wasm.br dist/frontend_bg.wasm
# mv -v dist/index.html.br dist/index.html
# mv -v dist/frontend.js.br dist/frontend.js
cp -rv dist/* ../backend/dist/
popd;

pushd backend;
cargo watch -x "run -- --release" &
ACTIX_PROC=$!
children+=($ACTIX_PROC)
popd;

wait $ACTIX_PROC
