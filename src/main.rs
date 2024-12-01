use std::{cmp::{max, min}, fs};

fn parse(data: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<(i32, i32)> = data
        .split("\n")
        .map(|line| line.split_once("   ").unwrap())
        .map(|(v1, v2)| (v1.parse().unwrap(), v2.parse().unwrap()))
        .collect();

    let mut first_col = lines.iter().map(|pair| pair.0).collect::<Vec<i32>>();
    first_col.sort();

    let mut second_col = lines.iter().map(|pair| pair.1).collect::<Vec<i32>>();
    second_col.sort();

    (first_col, second_col)
}

fn day1p2(data: &str) -> i32 {
    let (vec1, vec2) = parse(data);

    assert!(vec1.len() == vec2.len());

    let sum: i32 = vec1
        .iter()
        .map(|x| vec2.iter().filter(|&y| y == x).count() as i32 * *x)
        .sum();

    sum
}

fn day1p1(data: &str) -> i32 {
    let (vec1, vec2) = parse(data);

    assert!(vec1.len() == vec2.len());

    let sum: i32 = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| max(a, b) - min(a,b))
        .sum();
    sum
}

fn main() {
    let contents = fs::read_to_string("data/day1.data")
        .expect("Failed to read file");

    println!("Day 1: {}", day1p1(&contents[..]));
    println!("Day 2: {}", day1p2(&contents[..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(day1p1(data), 11);
    }

    #[test]
    fn part2_works() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(day1p2(data), 31);
    }
}
