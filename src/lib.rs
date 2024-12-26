use std::{
    ops::{Add, Mul, Rem, Sub},
    slice::Iter,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (d, 0) => d,
        (c, d) => gcd(d, c % d),
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

impl Mul<i64> for Position {
    type Output = Position;

    fn mul(self, _rhs: i64) -> Position {
        Position {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl Mul<&Position> for i64 {
    type Output = Position;

    fn mul(self, _rhs: &Position) -> Position {
        Position {
            x: _rhs.x * self,
            y: _rhs.y * self,
        }
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, _rhs: &'b Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<'a> Add<Position> for &'a Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Mul<Position> for i64 {
    type Output = Position;

    fn mul(self, _rhs: Position) -> Position {
        Position {
            x: _rhs.x * self,
            y: _rhs.y * self,
        }
    }
}

impl Rem<&Position> for Position {
    type Output = Position;

    fn rem(self, _rhs: &Position) -> Position {
        Position {
            x: _rhs.x.rem_euclid(self.x),
            y: _rhs.y.rem_euclid(self.y),
        }
    }
}
impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, _rhs: Position) -> Position {
        Position {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Move {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Move {
    pub fn advance_by(&self) -> Position {
        match self {
            Move::Up => Position { x: -1, y: 0 },
            Move::Right => Position { x: 0, y: 1 },
            Move::Down => Position { x: 1, y: 0 },
            Move::Left => Position { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}


impl Direction {
    pub fn from(c: char) -> Result<Direction, ()> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(()),
        }
    }

    pub fn advance(&self) -> (i32, i32) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }

    pub fn turn(&self) -> Self {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }

    pub fn advance_by(&self) -> Position {
        match self {
            Direction::North => Position { x: 0, y: 1 },
            Direction::East => Position { x: 1, y: 0 },
            Direction::South => Position { x: 0, y: -1 },
            Direction::West => Position { x: -1, y: 0 },
        }
    }
}

pub fn triangular(start: u64, end: u64) -> u64 {
    assert!(start <= end);
    if start == 0 {
        return end * (end + 1) / 2;
    }

    triangular(0, end) - triangular(0, start - 1)
}
