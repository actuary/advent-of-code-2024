use std::collections::{HashMap, HashSet};

use aoc2024::{Direction, Position};

struct Map {
    tiles: Vec<Vec<u64>>,
}

impl Map {
    fn find(&self, target: u64) -> Vec<Position> {
        let mut trailheads: Vec<Position> = Vec::new();

        for (i, row) in self.tiles.iter().enumerate() {
            for (j, &tile_value) in row.iter().enumerate() {
                if tile_value == target {
                    trailheads.push(Position {
                        x: i as i64,
                        y: j as i64,
                    });
                }
            }
        }
        trailheads
    }

    fn at(&self, position: &Position) -> u64 {
        self.tiles[position.x as usize][position.y as usize]
    }

    fn contains(&self, position: &Position) -> bool {
        (position.x >= 0)
            && (position.x < self.tiles.len() as i64)
            && (position.y >= 0)
            && (position.y < self.tiles[0].len() as i64)
    }
}

fn find_any_trail(
    position: Position,
    graph: &HashMap<Position, HashSet<Position>>,
    source: &mut HashSet<Position>,
) {
    for neighbour in graph.get(&position).unwrap().iter() {
        source.insert(*neighbour);
        find_any_trail(*neighbour, graph, source);
    }
}

fn find_unique_trails(
    position: Position,
    graph: &HashMap<Position, HashSet<Position>>,
    map: &Map,
) -> u64 {
    if map.at(&position) == 9 {
        return 1;
    }

    let mut result = 0;
    for neighbour in graph.get(&position).unwrap() {
        result += find_unique_trails(*neighbour, graph, map)
    }

    result
}

fn build_graph(map: &Map) -> HashMap<Position, HashSet<Position>> {
    let mut graph = HashMap::new();

    for (x, row) in map.tiles.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let position = Position {
                x: x as i64,
                y: y as i64,
            };
            graph.insert(position, HashSet::new());
            for direction in Direction::iterator() {
                let new_position = position + direction.advance_by();
                if !map.contains(&new_position) {
                    continue;
                }

                if map.at(&position) >= map.at(&new_position) {
                    continue;
                }

                if map.at(&new_position) - map.at(&position) == 1 {
                    graph.get_mut(&position).unwrap().insert(new_position);
                }
            }
        }
    }

    graph
}

fn parse(data: &str) -> Map {
    let positions: Vec<Vec<u64>> = data
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    Map { tiles: positions }
}

pub fn part1(data: &str) -> u64 {
    let map = parse(data);
    let graph = build_graph(&map);

    let mut score = 0;
    for trailhead in &map.find(0) {
        let mut reachable = HashSet::new();
        find_any_trail(*trailhead, &graph, &mut reachable);

        score += map
            .find(9)
            .iter()
            .filter(|trailtail| reachable.contains(trailtail))
            .count() as u64;
    }

    score
}

pub fn part2(data: &str) -> u64 {
    let map = parse(data);
    let graph = build_graph(&map);

    let mut score = 0;
    for trailtail in &map.find(0) {
        score += find_unique_trails(*trailtail, &graph, &map);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "0123
1234
8765
9876";
        assert_eq!(part1(data), 1);
    }

    #[test]
    fn part1_larger_works() {
        let data = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(data), 36);
        assert!(true);
    }

    #[test]
    fn part2_works() {
        let data = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part2(data), 81);
    }
}
