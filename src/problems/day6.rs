use std::collections::HashSet;

use aoc2024::Direction;

fn parse(data: &str) -> Vec<Vec<char>> {
    data.trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

enum TraverseResult {
    Cycle,
    Terminated(HashSet<(i32, i32)>),
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

fn inbounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    (x >= 0 && x < width) && (y >= 0 && y < height)
}

fn get_guard(map: &Vec<Vec<char>>) -> Result<Guard, ()> {
    for (x, row) in map.iter().enumerate() {
        for (y, &value) in row.iter().enumerate() {
            match Direction::from(value) {
                Ok(dir) => {
                    return Ok(Guard {
                        x: x as i32,
                        y: y as i32,
                        direction: dir,
                    });
                }
                Err(_) => {}
            };
        }
    }

    Err(())
}

fn traverse(map: &Vec<Vec<char>>) -> TraverseResult {
    let mut guard = get_guard(&map).unwrap();

    let mut visited: HashSet<Guard> = HashSet::new();
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(guard.clone());
    visited_pos.insert((guard.x, guard.y));

    while inbounds(guard.x, guard.y, map.len() as i32, map[0].len() as i32) {
        let (advance_x, advance_y) = guard.direction.advance();
        let test_guard = Guard {
            x: guard.x.clone() + advance_x,
            y: guard.y.clone() + advance_y,
            direction: guard.direction.clone(),
        };

        if visited.contains(&test_guard) {
            return TraverseResult::Cycle;
        } else if !inbounds(
            test_guard.x,
            test_guard.y,
            map.len() as i32,
            map[0].len() as i32,
        ) {
            guard = test_guard.clone()
        } else {
            match map[test_guard.x as usize][test_guard.y as usize] {
                '#' => {
                    guard = Guard {
                        x: guard.x,
                        y: guard.y,
                        direction: guard.direction.turn(),
                    };
                }
                '.' | '^' | 'v' | '>' | '<' => {
                    guard = Guard {
                        x: test_guard.x,
                        y: test_guard.y,
                        direction: test_guard.direction,
                    };
                    visited.insert(guard.clone());
                    visited_pos.insert((guard.x, guard.y));
                }
                _ => panic!("Unexpected map item.",),
            }
        }
    }

    TraverseResult::Terminated(visited_pos)
}

pub fn part1(data: &str) -> u32 {
    let map = parse(data);

    let result: u32 = match traverse(&map) {
        TraverseResult::Terminated(hash_set) => hash_set.len() as u32,
        TraverseResult::Cycle => panic!("Failed to complete!"),
    };

    result
}

pub fn part2(data: &str) -> u32 {
    // basically brute force with a small improvement - we only check the initial
    // visited path...
    //
    let mut map = parse(data);
    let guard = get_guard(&map).unwrap();

    let visited: HashSet<(i32, i32)> = match traverse(&map) {
        TraverseResult::Terminated(hash_set) => hash_set,
        TraverseResult::Cycle => panic!("Failed to complete!"),
    };

    let mut result: u32 = 0;
    let mut attempt: u32 = 0;
    for (a, b) in visited {
        attempt += 1;
        if !(guard.x == a && guard.y == b) {
            let tmp: char = map[a as usize][b as usize];
            map[a as usize][b as usize] = '#';

            match traverse(&map) {
                TraverseResult::Cycle => result += 1,
                TraverseResult::Terminated(_) => {}
            }

            map[a as usize][b as usize] = tmp;
        }
        println!("#{attempt}");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(data), 41);
    }

    #[test]
    fn part2_works() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part2(data), 6);
    }
}
