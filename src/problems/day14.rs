use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
}

struct Robot {
    p: Position,
    v: Position,
}

fn parse(data: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    data.trim()
        .split("\n")
        .map(|line| {
            let robot_match = re.captures(line).unwrap();
            Robot {
                p: Position {
                    x: robot_match["px"].parse().unwrap(),
                    y: robot_match["py"].parse().unwrap(),
                },
                v: Position {
                    x: robot_match["vx"].parse().unwrap(),
                    y: robot_match["vy"].parse().unwrap(),
                },
            }
        })
        .collect()
}

pub fn part1(data: &str) -> i64 {
    let robots = parse(data);

    let result: i64 = robots.len() as i64;
    result
}

pub fn part2(data: &str) -> i64 {
    let robots = parse(data);

    let result: i64 = robots.len() as i64;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part1(data), 12);
    }

    #[test]
    fn part2_works() {
        let data = "";
        assert_eq!(part2(data), 0);
    }
}
