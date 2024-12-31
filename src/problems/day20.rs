use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc2024::{Direction, Position};

fn parse(data: &str) -> Vec<Vec<char>> {
    let map = data
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    map
}

fn in_bounds(position: &Position, map: &Vec<Vec<char>>) -> bool {
    assert!(map.len() > 0 && map[0].len() > 0);

    position.x >= 0
        && position.x < map.len() as i64
        && position.y >= 0
        && position.y < map[0].len() as i64
}

fn at(position: &Position, map: &Vec<Vec<char>>) -> char {
    assert!(in_bounds(position, map));
    map[position.x as usize][position.y as usize]
}

fn find(target: char, map: &Vec<Vec<char>>) -> Option<Position> {
    for (x, row) in map.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            if *ch == target {
                return Some(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    None
}

#[derive(Eq, PartialEq)]
struct State {
    cost: i64,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    map: &Vec<Vec<char>>,
    start: &Position,
    end: &Position,
) -> Option<(i64, HashMap<Position, i64>)> {
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    dist.insert(*start, 0i64);

    queue.push(State {
        cost: 0,
        position: *start,
    });

    while let Some(State { cost, position }) = queue.pop() {
        if position == *end {
            return Some((cost, dist));
        }

        if cost > dist[&position] {
            continue;
        }

        for direction in Direction::iterator() {
            let next_position = position + direction.advance_by();

            if !in_bounds(&next_position, map) || at(&next_position, map) == '#' {
                continue;
            }

            let next = State {
                cost: cost + 1,
                position: next_position,
            };

            if !dist.contains_key(&next.position) || next.cost < dist[&next.position] {
                dist.insert(next.position, next.cost);
                queue.push(next);
            }
        }
    }

    None
}

fn filled_taxicab_dist_circle(
    centre: &Position,
    map: &Vec<Vec<char>>,
    max_cheat_length: i64,
) -> Vec<Position> {
    let mut result = Vec::new();

    for dist in 2..=max_cheat_length {
        for r in 0..dist {
            let new_positions = [
                Position {
                    x: centre.x + r,
                    y: centre.y + (dist - r),
                },
                Position {
                    x: centre.x + (dist - r),
                    y: centre.y - r,
                },
                Position {
                    x: centre.x - r,
                    y: centre.y - (dist - r),
                },
                Position {
                    x: centre.x - (dist - r),
                    y: centre.y + r,
                },
            ];

            for new_position in &new_positions {
                if in_bounds(&new_position, map) && at(&new_position, map) != '#' {
                    result.push(*new_position);
                }
            }
        }
    }

    result
}

fn taxicab(from: &Position, to: &Position) -> i64 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}

fn count_cost_savings_for_cheats(map: &Vec<Vec<char>>, threshold: i64, cheat_length: i64) -> i64 {
    let start = find('S', &map).unwrap();
    let end = find('E', &map).unwrap();

    let Some((cost, start_to_end_costs)) = dijkstra(&map, &start, &end) else {
        panic!("No solution found!");
    };

    let (_, end_to_start_costs) = dijkstra(&map, &end, &start).unwrap();

    let mut saving_count = 0;

    for x in 1..map.len() - 1 {
        for y in 1..map[0].len() - 1 {
            let position = Position {
                x: x as i64,
                y: y as i64,
            };

            if map[x][y] != '#' {
                let circle = filled_taxicab_dist_circle(&position, &map, cheat_length);
                for jump_position in &circle {
                    let cheat_length = taxicab(&position, jump_position);
                    let cost_with_cheat = start_to_end_costs[&position]
                        + end_to_start_costs[&jump_position]
                        + cheat_length;

                    let saving = cost - cost_with_cheat;

                    if saving >= threshold {
                        saving_count += 1;
                    }
                }
            }
        }
    }

    saving_count
}

pub fn part1(data: &str) -> i64 {
    let map = parse(data);

    count_cost_savings_for_cheats(&map, if cfg!(test) { 64 } else { 100 }, 2)
}

pub fn part2(data: &str) -> i64 {
    let map = parse(data);

    // after a bit of refactorign - took ages to realise that you were allowed
    // to keep cheating even after you reached an empty '.' space.
    count_cost_savings_for_cheats(&map, if cfg!(test) { 50 } else { 100 }, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        //1327
        assert_eq!(part1(data), 1);
    }

    #[test]
    fn part2_works() {
        let data = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(
            part2(data),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
