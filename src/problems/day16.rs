fn parse(data: &str) -> Vec<&str> {
    data
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
    fn part1_works_first_example() {
        let data = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part1(data), 7036);
    }

    #[test]
    fn part1_works_second_example() {
        let data = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part1(data), 11048);
    }

    #[test]
    fn part2_works() {
        let data = "";
        assert_eq!(part2(data), 0);
    }
}
