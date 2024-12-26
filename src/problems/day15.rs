use aoc2024::{Move, Position};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    Empty = 0,
    Robot = 1,
    Box = 2,
    Wall = 3,
    BoxLeft = 4,
    BoxRight = 5,
}

struct Game {
    tile_map: Vec<Vec<Tile>>,
    moves: Vec<Move>,
    robot: Position,
}

fn at<'a>(tile_map: &'a Vec<Vec<Tile>>, position: &Position) -> &'a Tile {
    &tile_map[position.x as usize][position.y as usize]
}

fn parse(data: &str) -> Game {
    let game: Vec<&str> = data.trim().split("\n\n").collect();

    let tile_map: Vec<Vec<Tile>> = game[0]
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    '@' => Tile::Robot,
                    'O' => Tile::Box,
                    '#' => Tile::Wall,
                    _ => panic!("Invalid tile {ch}"),
                })
                .collect()
        })
        .collect();

    let moves: Vec<Move> = game[1]
        .chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| match ch {
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            _ => panic!("Invalid move {ch}"),
        })
        .collect();

    let mut robot = None;
    for x in 0..tile_map.len() {
        for y in 0..tile_map[0].len() {
            if tile_map[x][y] == Tile::Robot {
                robot = Some(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    let Some(robot) = robot else {
        panic!("Robot not found!");
    };

    Game {
        tile_map,
        moves,
        robot,
    }
}

fn can_move_robot(to_move: &Position, tile_map: &Vec<Vec<Tile>>, mv: &Move) -> bool {
    let tile = *at(tile_map, to_move);
    let next_pos = to_move + mv.advance_by();

    if *mv == Move::Up || *mv == Move::Down {
        match tile {
            Tile::Empty => true,
            Tile::Robot => can_move_robot(&next_pos, tile_map, mv),
            Tile::BoxLeft => {
                let right_box = &next_pos + Move::Right.advance_by();

                can_move_robot(&next_pos, tile_map, mv) && can_move_robot(&right_box, tile_map, mv)
            }
            Tile::BoxRight => {
                let left_box = &next_pos + Move::Left.advance_by();

                can_move_robot(&next_pos, tile_map, mv) && can_move_robot(&left_box, tile_map, mv)
            }
            Tile::Wall => false,
            Tile::Box => can_move_robot(&next_pos, tile_map, mv),
        }
    } else {
        match tile {
            Tile::Empty => true,
            Tile::Robot => can_move_robot(&next_pos, tile_map, mv),
            Tile::BoxLeft => can_move_robot(&next_pos, tile_map, mv),
            Tile::BoxRight => can_move_robot(&next_pos, tile_map, mv),
            Tile::Wall => false,
            Tile::Box => can_move_robot(&next_pos, tile_map, mv),
        }
    }
}

fn move_robot(to_move: &Position, tile_map: &mut Vec<Vec<Tile>>, mv: &Move) {
    let tile = *at(tile_map, to_move);
    let next_pos = to_move + mv.advance_by();

    if *mv == Move::Up || *mv == Move::Down {
        match tile {
            Tile::Empty | Tile::Wall => (),
            Tile::Robot => {
                move_robot(&next_pos, tile_map, mv);
                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::Robot;
            }
            Tile::BoxLeft => {
                let right_box = to_move + Move::Right.advance_by();
                let next_right = &next_pos + Move::Right.advance_by();

                move_robot(&next_pos, tile_map, mv);
                move_robot(&next_right, tile_map, mv);

                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[right_box.x as usize][right_box.y as usize] = Tile::Empty;

                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::BoxLeft;
                tile_map[next_right.x as usize][next_right.y as usize] = Tile::BoxRight;
            }
            Tile::BoxRight => {
                let left_box = to_move + Move::Left.advance_by();
                let next_left = &next_pos + Move::Left.advance_by();

                move_robot(&next_left, tile_map, mv);
                move_robot(&next_pos, tile_map, mv);

                tile_map[left_box.x as usize][left_box.y as usize] = Tile::Empty;
                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;

                tile_map[next_left.x as usize][next_left.y as usize] = Tile::BoxLeft;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::BoxRight;
            }
            Tile::Box => {
                move_robot(&next_pos, tile_map, mv);

                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::Box;
            }
        }
    } else {
        match tile {
            Tile::Empty | Tile::Wall => (),
            Tile::Robot => {
                move_robot(&next_pos, tile_map, mv);

                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::Robot;
            }
            Tile::BoxLeft => {
                move_robot(&next_pos, tile_map, mv);

                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::BoxLeft;
            }
            Tile::BoxRight => {
                move_robot(&next_pos, tile_map, mv);

                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::BoxRight;
            }
            Tile::Box => {
                move_robot(&next_pos, tile_map, mv);

                tile_map[to_move.x as usize][to_move.y as usize] = Tile::Empty;
                tile_map[next_pos.x as usize][next_pos.y as usize] = Tile::Box;
            }
        }
    }
}

#[allow(dead_code)]
fn print_game(game: &Game) {
    for row in &game.tile_map {
        for tile in row {
            let ch = match tile {
                Tile::Empty => '.',
                Tile::Robot => '@',
                Tile::Wall => '#',
                Tile::Box => 'O',
                Tile::BoxLeft => '[',
                Tile::BoxRight => ']',
            };
            print!("{ch}");
        }
        print!("\n");
    }
}

pub fn part1(data: &str) -> i64 {
    let mut game = parse(data);

    let mut position = game.robot;

    for mv in &game.moves {
        if can_move_robot(&position, &game.tile_map, &mv) {
            move_robot(&position, &mut game.tile_map, &mv);
            position = &position + mv.advance_by();
        }
    }

    game.tile_map
        .iter()
        .enumerate()
        .map(|(x, row)| {
            let sum: i64 = row
                .iter()
                .enumerate()
                .map(|(y, tile)| {
                    let result: i64 = match tile {
                        Tile::Box => (100usize * x + y) as i64,
                        _ => 0i64,
                    };

                    result
                })
                .sum();

            sum
        })
        .sum()
}

fn expand_tilemap(tile_map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result: Vec<Vec<Tile>> = Vec::new();

    for row in tile_map {
        let mut new_row = Vec::new();
        for tile in row {
            match tile {
                Tile::Wall => {
                    new_row.push(Tile::Wall);
                    new_row.push(Tile::Wall);
                }
                Tile::Empty => {
                    new_row.push(Tile::Empty);
                    new_row.push(Tile::Empty);
                }
                Tile::Box => {
                    new_row.push(Tile::BoxLeft);
                    new_row.push(Tile::BoxRight);
                }
                Tile::Robot => {
                    new_row.push(Tile::Robot);
                    new_row.push(Tile::Empty);
                }
                _ => panic!("Bad tile found..."),
            }
        }
        result.push(new_row);
    }
    result
}

pub fn part2(data: &str) -> i64 {
    let mut game = parse(data);
    game.tile_map = expand_tilemap(&game.tile_map);

    let mut position = Position {
        x: game.robot.x,
        y: game.robot.y * 2,
    };

    for mv in &game.moves {
        if can_move_robot(&position, &game.tile_map, &mv) {
            move_robot(&position, &mut game.tile_map, &mv);
            position = &position + mv.advance_by();
        }
    }

    game.tile_map
        .iter()
        .enumerate()
        .map(|(x, row)| {
            let sum: i64 = row
                .iter()
                .enumerate()
                .map(|(y, tile)| {
                    let result: i64 = match tile {
                        Tile::BoxLeft => {
                            let x = x as i64;
                            let y = y as i64;
                            let x_coord = x;
                            let y_coord = y;

                            100 * x_coord + y_coord
                        }

                        _ => 0i64,
                    };

                    result
                })
                .sum();

            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_small_example() {
        let data = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(part1(data), 2028);
    }

    #[test]
    fn part1_works_larger_example() {
        let data = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part1(data), 10092);
    }

    #[test]
    fn part2_works_smaller() {
        let data = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!(part2(data), 618);
    }

    #[test]
    fn part2_works() {
        let data = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part2(data), 9021);
    }
}
