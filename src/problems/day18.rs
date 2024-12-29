use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc2024::{Direction, Position};

fn parse(data: &str) -> Vec<Position> {
    data.trim()
        .split("\n")
        .map(|line| {
            let pos = line.split_once(",").unwrap();
            Position {
                x: pos.0.parse().unwrap(),
                y: pos.1.parse().unwrap(),
            }
        })
        .collect()
}

fn in_bounds(position: &Position, map: &Vec<Vec<bool>>) -> bool {
    assert!(map.len() > 0 && map[0].len() > 0);

    position.x >= 0
        && position.x < map.len() as i64
        && position.y >= 0
        && position.y < map[0].len() as i64
}

fn at(position: &Position, map: &Vec<Vec<bool>>) -> bool {
    assert!(in_bounds(position, map));
    map[position.x as usize][position.y as usize]
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<bool>>) {
    for row in map.iter() {
        for &tile in row {
            if tile {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
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

fn dijkstra(map: &Vec<Vec<bool>>, start: Position, end: Position) -> Option<i64> {
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    dist.insert(start, 0i64);

    queue.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = queue.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        for direction in Direction::iterator() {
            let next_position = position + direction.advance_by();

            if !in_bounds(&next_position, map) || at(&next_position, map) {
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

pub fn part1(data: &str) -> i64 {
    let incoming = parse(data);

    let min_x = incoming.iter().map(|p| p.x).min().unwrap();
    let max_x = incoming.iter().map(|p| p.x).max().unwrap();
    let min_y = incoming.iter().map(|p| p.y).min().unwrap();
    let max_y = incoming.iter().map(|p| p.y).max().unwrap();

    let mut tile_map =
        vec![vec![false; (max_y - min_y) as usize + 1]; (max_x - min_x) as usize + 1];

    let mut max_bytes = 1024;
    if incoming.len() < 1024 {
        max_bytes = 12;
    }
    for position in &incoming[..max_bytes] {
        tile_map[position.x as usize][position.y as usize] = true;
    }

    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: (max_x - min_x),
        y: (max_y - min_y),
    };

    let Some(result) = dijkstra(&tile_map, start, end) else {
        panic!("No result found!");
    };

    result
}

pub fn part2(data: &str) -> String {
    let incoming = parse(data);

    let min_x = incoming.iter().map(|p| p.x).min().unwrap();
    let max_x = incoming.iter().map(|p| p.x).max().unwrap();
    let min_y = incoming.iter().map(|p| p.y).min().unwrap();
    let max_y = incoming.iter().map(|p| p.y).max().unwrap();

    let mut tile_map =
        vec![vec![false; (max_y - min_y) as usize + 1]; (max_x - min_x) as usize + 1];

    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: (max_x - min_x),
        y: (max_y - min_y),
    };

    let mut result: Option<Position> = None;

    for position in incoming.iter() {
        tile_map[position.x as usize][position.y as usize] = true;

        match dijkstra(&tile_map, start, end) {
            None => {
                result = Some(*position);
                break;
            }
            Some(_) => {}
        }
    }

    match result {
        Some(pos) => format!("{},{}", pos.x, pos.y),
        None => panic!("FAILED to find byteing-point"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part1(data), 22);
    }

    #[test]
    fn part2_works() {
        let data = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part2(data), "6,1");
    }
}
