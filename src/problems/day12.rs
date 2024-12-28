use std::collections::{HashMap, HashSet};

use aoc2024::{Position, Direction};

struct Map {
    tiles: Vec<Vec<char>>,
}

impl Map {
    fn at(&self, position: &Position) -> char {
        self.tiles[position.x as usize][position.y as usize]
    }

    fn in_bounds(&self, position: &Position) -> bool {
        (position.x >= 0)
            && (position.x < self.tiles.len() as i64)
            && (position.y >= 0)
            && (position.y < self.tiles[0].len() as i64)
    }
}

fn parse(data: &str) -> Map {
    Map {
        tiles: data
            .trim()
            .split("\n")
            .map(|line| line.chars().collect())
            .collect(),
    }
}

fn explore_region(start: &Position, map: &Map, visited: &mut HashSet<Position>) {
    for direction in Direction::iterator() {
        let new_position = start + direction.advance_by();
        if map.in_bounds(&new_position)
            && !visited.contains(&new_position)
            && map.at(&new_position) == map.at(start)
        {
            visited.insert(new_position);
            explore_region(&new_position, map, visited);
        }
    }
}

fn explore(map: &Map) -> HashMap<Position, HashSet<Position>> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut regions: HashMap<Position, HashSet<Position>> = HashMap::new();

    for (x, row) in map.tiles.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let position = Position {
                x: x as i64,
                y: y as i64,
            };

            if !visited.contains(&position) {
                regions.insert(position, HashSet::new());
                let mut region = regions.get_mut(&position).unwrap();
                region.insert(position);
                explore_region(&position, &map, &mut region);
                visited.extend(region.iter());
            }
        }
    }

    regions
}

fn region_area(region: &HashSet<Position>) -> i64 {
    region.len() as i64
}

fn region_perimeter(region: &HashSet<Position>, map: &Map) -> i64 {
    let mut perimeter: i64 = 0;

    for position in region {
        for direction in Direction::iterator() {
            let new_position = position + direction.advance_by();
            if !map.in_bounds(&new_position) || map.at(&new_position) != map.at(&position) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_edges(
    direction: Direction,
    region: &HashSet<Position>,
    min_p: &Position,
    max_p: &Position,
    map: &Map,
) -> i64 {
    let mut perimeter: i64 = 0;

    if (direction == Direction::North) || (direction == Direction::South) {
        for x in min_p.x..=max_p.x {
            let mut y = min_p.y;
            //do bottoms then tops
            while y <= max_p.y {
                let position = Position { x, y };
                let to_check = position + direction.advance_by();
                if region.contains(&position)
                    && (!map.in_bounds(&to_check) || !region.contains(&to_check))
                {
                    perimeter += 1;
                    let mut position = Position { x, y };
                    let mut to_check = position + direction.advance_by();
                    while y <= max_p.y
                        && region.contains(&position)
                        && (!map.in_bounds(&to_check) || !region.contains(&to_check))
                    {
                        y += 1;
                        position = Position { x, y };
                        to_check = position + direction.advance_by();
                    }
                } else {
                    y += 1;
                };
            }
        }
    } else {
        for y in min_p.y..=max_p.y {
            let mut x = min_p.x;
            while x <= max_p.x {
                let position = Position { x, y };
                let to_check = position + direction.advance_by();
                if region.contains(&position)
                    && (!map.in_bounds(&to_check) || !region.contains(&to_check))
                {
                    let mut position = Position { x, y };
                    let mut to_check = position + direction.advance_by();
                    while x <= max_p.x
                        && region.contains(&position)
                        && (!map.in_bounds(&to_check) || !region.contains(&to_check))
                    {
                        x += 1;
                        position = Position { x, y };
                        to_check = position + direction.advance_by();
                    }
                    perimeter += 1;
                } else {
                    x += 1;
                };
            }
        }
    }

    perimeter
}

fn region_perimeter_discounted(region: &HashSet<Position>, map: &Map) -> i64 {
    assert!(region.len() > 0);

    let min_p = Position {
        x: region.iter().map(|p| p.x).min().unwrap(),
        y: region.iter().map(|p| p.y).min().unwrap(),
    };

    let max_p = Position {
        x: region.iter().map(|p| p.x).max().unwrap(),
        y: region.iter().map(|p| p.y).max().unwrap(),
    };

    // first sort by (x, y) - this is the default
    let mut perimeter: i64 = 0;
    perimeter += calculate_edges(Direction::East, region, &min_p, &max_p, map);
    perimeter += calculate_edges(Direction::West, region, &min_p, &max_p, map);
    perimeter += calculate_edges(Direction::South, region, &min_p, &max_p, map);
    perimeter += calculate_edges(Direction::North, region, &min_p, &max_p, map);

    perimeter
}

pub fn part1(data: &str) -> i64 {
    let map = parse(data);
    let regions = explore(&map);

    let result: i64 = regions
        .values()
        .map(|values| region_area(values) * region_perimeter(values, &map))
        .sum();
    result
}

pub fn part2(data: &str) -> i64 {
    let map = parse(data);
    let regions = explore(&map);

    let result: i64 = regions
        .values()
        .map(|values| region_area(values) * region_perimeter_discounted(values, &map))
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part1(data), 1930);
    }

    #[test]
    fn part2_works_smaller() {
        let data = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part2(data), 80);
    }

    #[test]
    fn part2_works_es_and_xs() {
        let data = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(part2(data), 236);
    }

    #[test]
    fn part2_works_as_and_bs() {
        let data = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(part2(data), 368);
    }

    #[test]
    fn part2_works() {
        let data = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part2(data), 1206);
    }
}
