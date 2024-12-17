function get_problem() {
    curl "https://adventofcode.com/2024/day/$1/input" --cookie "session=${AOC_TOKEN};"
}
