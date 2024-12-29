fn parse(data: &str) -> Vec<&str> {
    data
        .trim()
        .split("\n")
        .collect()
}

pub fn part1(data: &str) -> i64 {
    let x = parse(data);
    
    let result: i64 = x.len() as i64;
    result
}

pub fn part2(data: &str) -> i64 {
    let x = parse(data);
    
    let result: i64 = x.len() as i64;
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
