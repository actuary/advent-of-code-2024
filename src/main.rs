use std::fs;

use problems::day17::{part1, part2};

pub mod problems;

fn main() {
    let contents = fs::read_to_string("data/day17.data").expect("Failed to read file");

    println!("Part 1: {}", part1(&contents[..]));
    println!("Part 2: {}", part2(&contents[..]));
}
