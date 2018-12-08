#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
    exit 1
fi

cargo run --bin "day$DAY" "${@:2}"
