#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
    exit 1
fi

RUST_BACKTRACE=0 cargo watch -x "test --bin day$DAY -- --nocapture"
