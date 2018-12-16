#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
    exit 1
fi

RUST_BACKTRACE=1 cargo run --features="$2" --bin "day$DAY" --release "${@:2}"
