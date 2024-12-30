use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
enum Stripe {
    White = 0,
    Blue,
    Black,
    Red,
    Green,
}

impl Stripe {
    fn from_char(ch: char) -> Stripe {
        match ch {
            'w' => Stripe::White,
            'u' => Stripe::Blue,
            'b' => Stripe::Black,
            'r' => Stripe::Red,
            'g' => Stripe::Green,
            _ => panic!("Unexpected stripe colour {ch}"),
        }
    }
}

fn parse(data: &str) -> (Vec<Vec<Stripe>>, Vec<Vec<Stripe>>) {
    let (towels, designs) = data.trim().split_once("\n\n").unwrap();

    let mut towels: Vec<Vec<Stripe>> = towels
        .split(", ")
        .map(|v| v.chars().map(|v| Stripe::from_char(v)).collect())
        .collect();

    towels.sort();

    let designs: Vec<Vec<Stripe>> = designs
        .split("\n")
        .map(|v| v.chars().map(|v| Stripe::from_char(v)).collect())
        .collect();

    (towels, designs)
}

fn is_possible(towels: &Vec<Vec<Stripe>>, design: &[Stripe]) -> bool {
    if design.len() == 0 {
        return true;
    }

    for towel in towels {
        if design.starts_with(towel) && is_possible(towels, &design[towel.len()..]) {
            return true;
        }
    }

    false
}

fn solve<'a>(
    towels: &Vec<Vec<Stripe>>,
    design: &'a [Stripe],
    i: usize,
    cache: &mut HashMap<(usize, &'a [Stripe]), i64>,
) -> i64 {
    if cache.contains_key(&(i, design)) {
        return cache[&(i, design)];
    }

    if design.len() > 0 && i >= towels.len() {
        cache.insert((i, design), 0);
        return 0;
    }

    if design.len() == 0 {
        cache.insert((i, design), 1);
        return 1;
    }

    let mut result: i64 = 0;
    let towel = &towels[i];
    if i < towels.len() && design.starts_with(towel) {
        result += solve(towels, &design[towel.len()..], 0, cache);
    }

    result += solve(towels, design, i + 1, cache);
    cache.insert((i, design), result);
    result
}

pub fn part1(data: &str) -> i64 {
    let (towels, designs) = parse(data);

    let mut count = 0;
    for design in designs {
        count += is_possible(&towels, &design) as i64;
    }
    count
}

pub fn part2(data: &str) -> i64 {
    let (towels, designs) = parse(data);

    let mut cache = HashMap::new();
    let mut count = 0;
    for design in &designs {
        let result = solve(&towels, &design, 0, &mut cache);
        count += result;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
        assert_eq!(part1(data), 6);
    }

    #[test]
    fn part2_works() {
        let data = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
        assert_eq!(part2(data), 16);
    }
}
