struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

fn parse(data: &str) -> Vec<Equation> {
    data.trim()
        .split("\n")
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();

            let numbers: Vec<u64> = right.split(" ").map(|v| v.parse().unwrap()).collect();

            Equation {
                test_value: left.parse().unwrap(),
                numbers,
            }
        })
        .collect()
}

fn solutions_helper(
    numbers: &mut Vec<u64>,
    target: u64,
    result: u64,
    bin_ops: &[fn(u64, u64) -> u64],
) -> u64 {
    if (numbers.len() == 0) && (result == target) {
        return 1;
    } else if numbers.len() == 0 {
        return 0;
    }

    let head = numbers.pop().unwrap();
    let number_of_solutions = bin_ops
        .iter()
        .map(|bin_op| solutions_helper(numbers, target, bin_op(result, head), bin_ops))
        .sum();
    numbers.push(head);
    number_of_solutions
}

fn solutions(equation: &Equation, bin_ops: &[fn(u64, u64) -> u64]) -> u64 {
    let mut numbers: Vec<u64> = equation.numbers.clone();
    numbers.reverse();

    let head = numbers.pop().unwrap();

    solutions_helper(&mut numbers, equation.test_value, head, &bin_ops[..])
}

pub fn part1(data: &str) -> u64 {
    let equations = parse(data);

    let bin_ops: [fn(u64, u64) -> u64; 2] = [
        |x, y| x + y,
        |x, y| x * y,
    ];

    let result: u64 = equations
        .iter()
        .filter(|eq| solutions(&eq, &bin_ops[..]) > 0)
        .map(|eq| eq.test_value)
        .sum();

    result
}

pub fn part2(data: &str) -> u64 {
    let equations = parse(data);

    let bin_ops: [fn(u64, u64) -> u64; 3] = [
        |x, y| x + y,
        |x, y| x * y,
        |x, y| format!("{x}{y}").parse().unwrap()
    ];


    let result: u64 = equations
        .iter()
        .filter(|eq| solutions(&eq, &bin_ops[..]) > 0)
        .map(|eq| eq.test_value)
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(data), 3749);
    }

    #[test]
    fn part2_works() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part2(data), 11387);
    }
}
