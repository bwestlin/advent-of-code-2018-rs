#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
fi

cargo run --bin "day$DAY"
