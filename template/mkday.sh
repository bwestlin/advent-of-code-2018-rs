#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
fi

DST="src/day$DAY/day$DAY.rs"
mkdir -p "src/day$DAY/"
sed "s/xDAYx/$DAY/g" < template/day.rs > "$DST"
echo "$DST created"
