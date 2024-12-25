use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (d, 0) => d,
        (c, d) => gcd(d, c % d),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

fn parse(data: &str) -> Vec<ClawMachine> {
    let re_button = Regex::new(r": X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_prize = Regex::new(r": X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    data.trim()
        .split("\n\n")
        .map(|group| {
            let lines: Vec<&str> = group.split("\n").collect();
            assert_eq!(lines.len(), 3);

            let a_match = re_button.captures(lines[0]).unwrap();
            let b_match = re_button.captures(lines[1]).unwrap();
            let prize_match = re_prize.captures(lines[2]).unwrap();
            ClawMachine {
                button_a: Position {
                    x: a_match["x"].parse().unwrap(),
                    y: a_match["y"].parse().unwrap(),
                },
                button_b: Position {
                    x: b_match["x"].parse().unwrap(),
                    y: b_match["y"].parse().unwrap(),
                },
                prize: Position {
                    x: prize_match["x"].parse().unwrap(),
                    y: prize_match["y"].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn solve_machine(machine: &ClawMachine) -> Option<(i64, i64)> {
    let lcm_a = lcm(machine.button_a.x, machine.button_a.y);

    let fac = (lcm_a / machine.button_a.x) * machine.button_b.x
        - (lcm_a / machine.button_a.y) * machine.button_b.y;

    let prize = (lcm_a / machine.button_a.x) * machine.prize.x
        - (lcm_a / machine.button_a.y) * machine.prize.y;

    if prize % fac == 0 {
        let b = prize / fac;

        if (machine.prize.x - b * machine.button_b.x) % machine.button_a.x == 0 {
            let a = (machine.prize.x - b * machine.button_b.x) / machine.button_a.x;
            return Some((a, b));
        }
    }

    None
}

pub fn part1(data: &str) -> i64 {
    let machines = parse(data);

    let result = machines
        .iter()
        .map(|machine| {
            let (a, b) = solve_machine(machine).unwrap_or((0, 0));
            3 * a + b
        })
        .sum();
    result
}

pub fn part2(data: &str) -> i64 {
    let mut machines = parse(data);

    for machine in machines.iter_mut() {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
    }

    //failed: 98958951401149

    let result = machines
        .iter()
        .map(|machine| {
            let (a, b) = solve_machine(machine).unwrap_or((0, 0));
            3 * a + b
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part1(data), 480);
    }

    #[test]
    fn part2_works() {
        let data = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part2(data), 875318608908);
    }

    #[test]
    fn part2_works_example() {
        let data = "Button A: X+93, Y+93
Button B: X+11, Y+90
Prize: X=4103, Y=11529";
        assert_eq!(part2(data), 0);
    }
}
