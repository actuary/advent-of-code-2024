use std::fs;

use problems::day4::{part1, part2};

pub mod problems;

fn main() {
    let contents = fs::read_to_string("data/day4.data").expect("Failed to read file");

    println!("Part 1: {}", part1(&contents[..]));
    println!("Part 2: {}", part2(&contents[..]));
}