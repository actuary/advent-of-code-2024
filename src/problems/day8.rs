use std::{
    cmp::min, collections::{HashMap, HashSet}, ops
};

struct Roof {
    width: u64,
    height: u64,
    antennae: HashMap<char, Vec<Position>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, _rhs: Position) -> Position {
        Position {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::Mul<i64> for Position {
    type Output = Position;

    fn mul(self, _rhs: i64) -> Position {
        Position {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

fn parse(data: &str) -> Roof {
    let map: Vec<Vec<char>> = data
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut antennae: HashMap<char, Vec<Position>> = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != '.' {
                if !antennae.contains_key(&ch) {
                    antennae.insert(ch, Vec::new());
                }

                let position = Position {
                    x: i as i64,
                    y: j as i64,
                };
                antennae.get_mut(&ch).unwrap().push(position);
            }
        }
    }

    Roof {
        width: map[0].len() as u64,
        height: map.len() as u64,
        antennae,
    }
}

fn antinode(
    antenna_i: &Position,
    antenna_j: &Position,
    width: u64,
    height: u64,
) -> (Option<Position>, Option<Position>) {
    assert_ne!(antenna_i, antenna_j);

    let antinode_a = (*antenna_i - *antenna_j) + *antenna_i;
    let antinode_b = (*antenna_j - *antenna_i) + *antenna_j;

    (
        if (antinode_a.x >= 0 && antinode_a.x < height as i64)
            && (antinode_a.y >= 0 && antinode_a.y < width as i64)
        {
            Some(antinode_a)
        } else {
            None
        },
        if (antinode_b.x >= 0 && antinode_b.x < height as i64)
            && (antinode_b.y >= 0 && antinode_b.y < width as i64)
        {
            Some(antinode_b)
        } else {
            None
        },
    )
}

fn antinode_any(
    antenna_i: &Position,
    antenna_j: &Position,
    width: u64,
    height: u64,
) -> Vec<Position> {
    assert_ne!(antenna_i, antenna_j);

    let mut antinodes: Vec<Position> = Vec::new();
    antinodes.push(*antenna_i);

    let step = *antenna_i - *antenna_j;

    let mut forward: i64 = 0;
    let mut backward: i64 = 0;
    if step.x < 0 && step.y > 0 {
        forward = min(antenna_i.x / -step.x, (width as i64 - 1 - antenna_i.y) / step.y);
        backward = min((height as i64 - 1 - antenna_i.x) / -step.x, antenna_i.y / step.y);

    } else if step.x > 0 && step.y > 0 {
        forward = min((height as i64 - 1 - antenna_i.x) / step.x, (width as i64 - 1 - antenna_i.y) / step.y);
        backward = min(antenna_i.x / step.x, antenna_i.y / step.y);

    } else if step.x < 0 && step.y < 0 {
        forward = min(antenna_i.x / -step.x, antenna_i.y / -step.y);
        backward = min((height as i64 - 1 - antenna_i.x) / -step.x, (width as i64 - 1 - antenna_i.y) / -step.y);

    } else if step.x > 0 && step.y < 0 {
        forward = min((height as i64 - 1 - antenna_i.x) / step.x, antenna_i.y / -step.y);
        backward = min(antenna_i.x / step.x, (width as i64 - 1 - antenna_i.y) / -step.y);
    }

    assert!(forward >= 0);
    assert!(backward >= 0);

    if forward > 0 {
        for i in 1..=forward {
            antinodes.push(*antenna_i + (step * i));
        }
    }

    if backward > 0 {
        for i in 1..=backward {
            antinodes.push(*antenna_i - (step * i));
        }
    }

    antinodes
}


pub fn part1(data: &str) -> u64 {
    let roof = parse(data);

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, v) in &roof.antennae {
        for (i, antenna_i) in v.iter().enumerate().take(v.len() - 1) {
            for antenna_j in v.iter().skip(i + 1) {
                let (node_a, node_b) = antinode(antenna_i, antenna_j, roof.width, roof.height);

                match node_a {
                    Some(node) => {
                        antinodes.insert(node);
                    }
                    None => (),
                }

                match node_b {
                    Some(node) => {
                        antinodes.insert(node);
                    }
                    None => (),
                }

            }
        }
    }

    antinodes.len() as u64
}

pub fn part2(data: &str) -> u64 {
    let roof = parse(data);

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, v) in &roof.antennae {
        for (i, antenna_i) in v.iter().enumerate().take(v.len() - 1) {
            for antenna_j in v.iter().skip(i + 1) {
                let antinodes_for_frequency = antinode_any(antenna_i, antenna_j, roof.width, roof.height);
                antinodes.extend(antinodes_for_frequency.iter());
            }
        }
    }

    antinodes.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(data), 14);
    }

    #[test]
    fn part2_works() {
        let data = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part2(data), 34);
    }
}
