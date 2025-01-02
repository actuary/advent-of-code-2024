use std::collections::HashMap;

fn numpad_to_arrow(chars: (char, char)) -> Vec<char> {
    match chars {
        // see scripts/keypad.py
        ('A', 'A') => vec!['A'],
        ('A', '0') => vec!['<', 'A'],
        ('A', '1') => vec!['^', '<', '<', 'A'],
        ('A', '2') => vec!['<', '^', 'A'],
        ('A', '3') => vec!['^', 'A'],
        ('A', '4') => vec!['^', '^', '<', '<', 'A'],
        ('A', '5') => vec!['<', '^', '^', 'A'],
        ('A', '6') => vec!['^', '^', 'A'],
        ('A', '7') => vec!['^', '^', '^', '<', '<', 'A'],
        ('A', '8') => vec!['<', '^', '^', '^', 'A'],
        ('A', '9') => vec!['^', '^', '^', 'A'],
        ('0', 'A') => vec!['>', 'A'],
        ('0', '0') => vec!['A'],
        ('0', '1') => vec!['^', '<', 'A'],
        ('0', '2') => vec!['^', 'A'],
        ('0', '3') => vec!['^', '>', 'A'],
        ('0', '4') => vec!['^', '^', '<', 'A'],
        ('0', '5') => vec!['^', '^', 'A'],
        ('0', '6') => vec!['^', '^', '>', 'A'],
        ('0', '7') => vec!['^', '^', '^', '<', 'A'],
        ('0', '8') => vec!['^', '^', '^', 'A'],
        ('0', '9') => vec!['^', '^', '^', '>', 'A'],
        ('1', 'A') => vec!['>', '>', 'v', 'A'],
        ('1', '0') => vec!['>', 'v', 'A'],
        ('1', '1') => vec!['A'],
        ('1', '2') => vec!['>', 'A'],
        ('1', '3') => vec!['>', '>', 'A'],
        ('1', '4') => vec!['^', 'A'],
        ('1', '5') => vec!['^', '>', 'A'],
        ('1', '6') => vec!['^', '>', '>', 'A'],
        ('1', '7') => vec!['^', '^', 'A'],
        ('1', '8') => vec!['^', '^', '>', 'A'],
        ('1', '9') => vec!['^', '^', '>', '>', 'A'],
        ('2', 'A') => vec!['v', '>', 'A'],
        ('2', '0') => vec!['v', 'A'],
        ('2', '1') => vec!['<', 'A'],
        ('2', '2') => vec!['A'],
        ('2', '3') => vec!['>', 'A'],
        ('2', '4') => vec!['<', '^', 'A'],
        ('2', '5') => vec!['^', 'A'],
        ('2', '6') => vec!['^', '>', 'A'],
        ('2', '7') => vec!['<', '^', '^', 'A'],
        ('2', '8') => vec!['^', '^', 'A'],
        ('2', '9') => vec!['^', '^', '>', 'A'],
        ('3', 'A') => vec!['v', 'A'],
        ('3', '0') => vec!['<', 'v', 'A'],
        ('3', '1') => vec!['<', '<', 'A'],
        ('3', '2') => vec!['<', 'A'],
        ('3', '3') => vec!['A'],
        ('3', '4') => vec!['<', '<', '^', 'A'],
        ('3', '5') => vec!['<', '^', 'A'],
        ('3', '6') => vec!['^', 'A'],
        ('3', '7') => vec!['<', '<', '^', '^', 'A'],
        ('3', '8') => vec!['<', '^', '^', 'A'],
        ('3', '9') => vec!['^', '^', 'A'],
        ('4', 'A') => vec!['>', '>', 'v', 'v', 'A'],
        ('4', '0') => vec!['>', 'v', 'v', 'A'],
        ('4', '1') => vec!['v', 'A'],
        ('4', '2') => vec!['v', '>', 'A'],
        ('4', '3') => vec!['v', '>', '>', 'A'],
        ('4', '4') => vec!['A'],
        ('4', '5') => vec!['>', 'A'],
        ('4', '6') => vec!['>', '>', 'A'],
        ('4', '7') => vec!['^', 'A'],
        ('4', '8') => vec!['^', '>', 'A'],
        ('4', '9') => vec!['^', '>', '>', 'A'],
        ('5', 'A') => vec!['v', 'v', '>', 'A'],
        ('5', '0') => vec!['v', 'v', 'A'],
        ('5', '1') => vec!['<', 'v', 'A'],
        ('5', '2') => vec!['v', 'A'],
        ('5', '3') => vec!['v', '>', 'A'],
        ('5', '4') => vec!['<', 'A'],
        ('5', '5') => vec!['A'],
        ('5', '6') => vec!['>', 'A'],
        ('5', '7') => vec!['<', '^', 'A'],
        ('5', '8') => vec!['^', 'A'],
        ('5', '9') => vec!['^', '>', 'A'],
        ('6', 'A') => vec!['v', 'v', 'A'],
        ('6', '0') => vec!['<', 'v', 'v', 'A'],
        ('6', '1') => vec!['<', '<', 'v', 'A'],
        ('6', '2') => vec!['<', 'v', 'A'],
        ('6', '3') => vec!['v', 'A'],
        ('6', '4') => vec!['<', '<', 'A'],
        ('6', '5') => vec!['<', 'A'],
        ('6', '6') => vec!['A'],
        ('6', '7') => vec!['<', '<', '^', 'A'],
        ('6', '8') => vec!['<', '^', 'A'],
        ('6', '9') => vec!['^', 'A'],
        ('7', 'A') => vec!['>', '>', 'v', 'v', 'v', 'A'],
        ('7', '0') => vec!['>', 'v', 'v', 'v', 'A'],
        ('7', '1') => vec!['v', 'v', 'A'],
        ('7', '2') => vec!['v', 'v', '>', 'A'],
        ('7', '3') => vec!['v', 'v', '>', '>', 'A'],
        ('7', '4') => vec!['v', 'A'],
        ('7', '5') => vec!['v', '>', 'A'],
        ('7', '6') => vec!['v', '>', '>', 'A'],
        ('7', '7') => vec!['A'],
        ('7', '8') => vec!['>', 'A'],
        ('7', '9') => vec!['>', '>', 'A'],
        ('8', 'A') => vec!['v', 'v', 'v', '>', 'A'],
        ('8', '0') => vec!['v', 'v', 'v', 'A'],
        ('8', '1') => vec!['<', 'v', 'v', 'A'],
        ('8', '2') => vec!['v', 'v', 'A'],
        ('8', '3') => vec!['v', 'v', '>', 'A'],
        ('8', '4') => vec!['<', 'v', 'A'],
        ('8', '5') => vec!['v', 'A'],
        ('8', '6') => vec!['v', '>', 'A'],
        ('8', '7') => vec!['<', 'A'],
        ('8', '8') => vec!['A'],
        ('8', '9') => vec!['>', 'A'],
        ('9', 'A') => vec!['v', 'v', 'v', 'A'],
        ('9', '0') => vec!['<', 'v', 'v', 'v', 'A'],
        ('9', '1') => vec!['<', '<', 'v', 'v', 'A'],
        ('9', '2') => vec!['<', 'v', 'v', 'A'],
        ('9', '3') => vec!['v', 'v', 'A'],
        ('9', '4') => vec!['<', '<', 'v', 'A'],
        ('9', '5') => vec!['<', 'v', 'A'],
        ('9', '6') => vec!['v', 'A'],
        ('9', '7') => vec!['<', '<', 'A'],
        ('9', '8') => vec!['<', 'A'],
        ('9', '9') => vec!['A'],
        _ => panic!("Unknown mapping {} to {}!", chars.0, chars.1),
    }
}

fn arrow_to_arrow(chars: &[char]) -> Vec<char> {
    assert!(chars.len() == 2);

    match (chars[0], chars[1]) {
        // see scripts/keypad.py
        ('A', 'A') => vec!['A'],
        ('A', '>') => vec!['v', 'A'],
        ('A', '^') => vec!['<', 'A'],
        ('A', '<') => vec!['v', '<', '<', 'A'],
        ('A', 'v') => vec!['<', 'v', 'A'],
        ('>', 'A') => vec!['^', 'A'],
        ('>', '>') => vec!['A'],
        ('>', '^') => vec!['<', '^', 'A'],
        ('>', '<') => vec!['<', '<', 'A'],
        ('>', 'v') => vec!['<', 'A'],
        ('^', 'A') => vec!['>', 'A'],
        ('^', '>') => vec!['v', '>', 'A'],
        ('^', '^') => vec!['A'],
        ('^', '<') => vec!['v', '<', 'A'],
        ('^', 'v') => vec!['v', 'A'],
        ('<', 'A') => vec!['>', '>', '^', 'A'],
        ('<', '>') => vec!['>', '>', 'A'],
        ('<', '^') => vec!['>', '^', 'A'],
        ('<', '<') => vec!['A'],
        ('<', 'v') => vec!['>', 'A'],
        ('v', 'A') => vec!['^', '>', 'A'],
        ('v', '>') => vec!['>', 'A'],
        ('v', '^') => vec!['^', 'A'],
        ('v', '<') => vec!['<', 'A'],
        ('v', 'v') => vec!['A'],
        _ => panic!("Unknown mapping {} to {}!", chars[0], chars[1]),
    }
}

fn shortest_sequence_between_two(
    start: char,
    end: char,
    robots: u64,
    ocache: &mut HashMap<(char, char, u64), u64>,
) -> u64 {
    if ocache.contains_key(&(start, end, robots)) {
        return *ocache.get(&(start, end, robots)).unwrap();
    }

    if robots == 0 {
        return 1;
    }

    let result = shortest_sequence(arrow_to_arrow(&[start, end]), robots - 1, ocache);

    ocache.insert((start, end, robots), result);

    result
}

fn shortest_sequence(
    chars: Vec<char>,
    robots: u64,
    ocache: &mut HashMap<(char, char, u64), u64>,
) -> u64 {
    let mut result: u64 = 0;
    let mut start = 'A';

    for &end in chars.iter() {
        result += shortest_sequence_between_two(start, end, robots, ocache);
        start = end;
    }

    result
}

fn cumulative_shortest_sequence(code: &[char], count_robots: u64) -> u64 {
    let code: Vec<char> = ['A'].iter().cloned().chain(code.iter().cloned()).collect();
    let robot: Vec<char> = code
        .windows(2)
        .map(|slice| numpad_to_arrow((slice[0], slice[1])))
        .flatten()
        .collect();

    let mut ocache = HashMap::new();
    shortest_sequence(robot, count_robots, &mut ocache)
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1(data: &str) -> u64 {
    let codes = parse(data);

    let sum: u64 = codes
        .iter()
        .map(|code| {
            cumulative_shortest_sequence(&code, 2)
                * code[..code.len() - 1]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
        })
        .sum();

    // too high: 164684
    // too high: 161472
    // wrong: 156544
    sum
}

pub fn part2(data: &str) -> u64 {
    let codes = parse(data);

    let sum: u64 = codes
        .iter()
        .map(|code| {
            cumulative_shortest_sequence(&code, 25)
                * code[..code.len() - 1]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "029A
980A
179A
456A
379A";
        assert_eq!(part1(data), 126384);
    }

    #[test]
    fn part2_works() {
        let data = "029A
980A
179A
456A
379A";
        assert_eq!(part2(data), 154115708116294);
    }
}
