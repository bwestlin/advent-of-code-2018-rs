#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
    exit 1
fi

DST="src/day$DAY/day$DAY.rs"
mkdir -p "src/day$DAY/"
sed "s/xDAYx/day$DAY/g" < template/day.rs > "$DST"
echo "$DST created"

echo "
[[bin]]
name = \"day$DAY\"
path = \"$DST\"" >> Cargo.toml