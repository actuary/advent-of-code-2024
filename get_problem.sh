#!/bin/bash

function get-aoc-problem() {
    set -x
    curl "https://adventofcode.com/2024/day/$1/input" --cookie "session=${AOC_TOKEN};"
}

function new-aoc() {
    set -x
    get-aoc-problem $1 > "data/day$1.data"
    cp "src/problems/template.rs" "src/problems/day$1.rs"
    echo "pub mod day$1;">>src/problems/mod.rs
    sed -E -i 's/day[[:digit:]]+/day'"$1"'/g' src/main.rs
}

