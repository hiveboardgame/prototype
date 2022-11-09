#!/usr/bin/env bash

export TRUNK_SERVE_PORT=8080
export ACTIX_PORT=8081

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

pushd backend;
cargo watch -x "run" &
ACTIX_PROC=$!
children+=($ACTIX_PROC)
popd;

pushd frontend;
trunk serve &
YEW_PROCESS=$!
children+=($YEW_PROCESS)
popd;

wait $ACTIX_PROC
