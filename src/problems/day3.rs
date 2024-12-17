use regex::Regex;

fn muls(data: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: u64 = re.captures_iter(data).map(|caps| {
        (caps[1].parse::<u64>().unwrap(), caps[2].parse::<u64>().unwrap())
    }).map(|(a, b)| a * b).sum();

    result
}

pub fn part1(data: &str) -> u64 {
    muls(data)
}

pub fn part2(data: &str) -> u64 {
    let splits: Vec<&str> = data.split("don't()").collect();

    let mut result: u64 = 0;

    result += muls(splits[0]);

    for group in splits.iter().skip(1) {
        for enabled in group.split("do()").skip(1) {
            result += muls(enabled);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(data), 161);
    }

    #[test]
    fn part2_works() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(data), 48);
    }
}
