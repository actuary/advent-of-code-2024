use std::cmp::{max, min};

fn parse(data: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<(i32, i32)> = data
        .split("\n")
        .collect();
}

pub fn part1(data: &str) -> i32 {
    let x = parse(data);
    
    let result: i32 = 0;
    result
}

pub fn part2(data: &str) -> i32 {
    let x = parse(data);
    
    let result: i32 = 0;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "";
        assert_eq!(part1(data), 0);
    }

    #[test]
    fn part2_works() {
        let data = "";
        assert_eq!(part2(data), 0);
    }
}
