use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc2024::Direction;
use aoc2024::Position;

#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Start = 2,
    End = 3,
    Walked = 4, 
}

fn in_bounds(position: &Position, map: &Vec<Vec<Tile>>) -> bool {
    assert!(map.len() > 0 && map[0].len() > 0);

    position.x >= 0
        && position.x < map.len() as i64
        && position.y >= 0
        && position.y < map[0].len() as i64
}

fn at(position: &Position, map: &Vec<Vec<Tile>>) -> Tile {
    assert!(in_bounds(position, map));
    map[position.x as usize][position.y as usize]
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
struct State {
    position: Position,
    cost: i64,
    direction: Direction,
    path: Vec<Position>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn is_opposite(a: Direction, b: Direction) -> bool {
    (a == Direction::South && b == Direction::North)
        || (a == Direction::North && b == Direction::South)
        || (a == Direction::East && b == Direction::West)
        || (a == Direction::West && b == Direction::East)
}

fn turn_cost(a: Direction, b: Direction) -> i64 {
    if a == b {
        1
    } else if is_opposite(a, b) {
        2000
    } else {
        1000
    }
}

fn get_neighbours(direction: Direction, position: Position) -> Vec<(Direction, Position)> {
    let mut neighbours = vec![(direction, position + direction.advance_by())];

    for dir in Direction::iterator() {
        // going the opposite way is never going to be better, so make
        // it impossible.
        if direction != *dir && !is_opposite(*dir, direction) {
            neighbours.push((*dir, position));
        }
    }

    neighbours
}

fn dijkstra(map: &Vec<Vec<Tile>>, start: &Position, end: &Position) -> Option<(i64, i64)> {
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    dist.insert((*start, Direction::East), 0i64);

    let mut paths = Vec::new();

    queue.push(State {
        cost: 0,
        position: *start,
        direction: Direction::East,
        path: vec![*start]
    });

    let mut min_cost = i64::MAX;

    // main mistake was taking the short cut of combining turn + move, instead
    // of leaving separate. This made me end up with less paths than I should have.
    // Of course this was going to be the case, as sometimes we have cris-crossing
    // paths...
    //
    // below has them being separate, which I needed a hint for.
    while let Some(State {
        cost,
        position,
        direction,
        path
    }) = queue.pop()
    {
        if cost > *dist.get(&(position, direction)).unwrap() {
            continue;
        }

        for (next_direction, next_position) in get_neighbours(direction, position) {
            if at(&next_position, map) == Tile::Wall  {
                continue;
            }

            let mut next_path = path.clone();
            next_path.push(next_position);

            let next = State {
                cost: cost + turn_cost(direction, next_direction),
                position: next_position,
                direction: next_direction,
                path: next_path.clone()
            };

            let current_cost = *dist.entry((next.position, next_direction)).or_insert(i64::MAX);
            if next.cost <= current_cost {
                if &next_position == end && next.cost <= min_cost {
                    if next.cost < min_cost {
                        paths.clear();
                        min_cost = next.cost;
                    }
                    paths.push(next_path);
                }

                dist.insert((next.position, next_direction), next.cost);
                queue.push(next);
            }
        }
    }

    if min_cost < i64::MAX {
        let unique_seats: HashSet<Position> = paths.into_iter().flatten().collect();
        return Some((min_cost, unique_seats.len() as i64));
    }

    None
}

fn parse(data: &str) -> Vec<Vec<Tile>> {
    data.trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => panic!("Invalid tile char {ch}"),
                })
                .collect()
        })
        .collect()
}


#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Tile>>) {
    for row in map.iter() {
        for tile in row {
            let ch = match tile {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Start => 'S',
                Tile::End => 'E',
                Tile::Walked => 'O',
            };
            print!("{ch}");
        }
        print!("\n");
    }
    print!("\n");
}

pub fn part1(data: &str) -> i64 {
    let map = parse(data);

    let start = Position {
        x: map.len() as i64 - 2,
        y: 1
    };

    let end = Position {
        x: 1,
        y: map[0].len() as i64 - 2
    };

    if let Some((value, _)) = dijkstra(&map, &start, &end) {
        value
    } else {
        panic!("No solution found!");
    }
}

pub fn part2(data: &str) -> i64 {
    let map = parse(data);

    let start = Position {
        x: map.len() as i64 - 2,
        y: 1
    };

    let end = Position {
        x: 1,
        y: map[0].len() as i64 - 2
    };

    if let Some((_, seats)) = dijkstra(&map, &start, &end) {
        return seats;
    } else {
        panic!("No solution found!");
    }
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
    fn part2_works_first_example() {
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
        assert_eq!(part2(data), 45);
    }

    #[test]
    fn part2_works_second_example() {
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
        assert_eq!(part2(data), 64);
    }
}
