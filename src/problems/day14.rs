use aoc2024::Position;
use regex::Regex;

struct Robot {
    p: Position,
    v: Position,
}

fn parse(data: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    data.trim()
        .split("\n")
        .map(|line| {
            let robot_match = re.captures(line).unwrap();
            Robot {
                p: Position {
                    x: robot_match["px"].parse().unwrap(),
                    y: robot_match["py"].parse().unwrap(),
                },
                v: Position {
                    x: robot_match["vx"].parse().unwrap(),
                    y: robot_match["vy"].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn advance_robot(robot: &Robot, map_dim: &Position, t: i64) -> Position {
    let position = robot.p + t * robot.v;
    Position {
        x: position.x.rem_euclid(map_dim.x),
        y: position.y.rem_euclid(map_dim.y),
    }
}

fn count_robots(start: &Position, end: &Position, robots: &Vec<Position>) -> i64 {
    let mut count: i64 = 0;
    for &robot in robots.iter() {
        if robot.x >= start.x && robot.x <= end.x && robot.y >= start.y && robot.y <= end.y {
            count += 1;
        }
    }

    count
}

pub fn part1(data: &str) -> i64 {
    let robots = parse(data);
    let map_dim = Position { x: 101, y: 103 };
    // let map_dim = Position { x: 11, y: 7 };
    let midpoint = Position {
        x: (&map_dim.x - 1) / 2,
        y: (&map_dim.y - 1) / 2,
    };

    let final_positions: Vec<Position> = robots
        .iter()
        .map(|robot| advance_robot(&robot, &map_dim, 100))
        .filter(|&position| position.x != midpoint.x && position.y != midpoint.y)
        .collect();

    let quad_a_start = Position { x: 0, y: 0 };

    let quad_b_start = Position {
        x: 0,
        y: midpoint.y,
    };

    let quad_b_end = Position {
        x: midpoint.x,
        y: map_dim.y - 1,
    };

    let quad_c_start = Position {
        x: midpoint.x,
        y: 0,
    };

    let quad_c_end = Position {
        x: map_dim.x - 1,
        y: midpoint.y,
    };

    let quad_d_end = Position {
        x: &map_dim.x - 1,
        y: &map_dim.y - 1,
    };

    let q1 = count_robots(&quad_a_start, &midpoint, &final_positions);
    let q2 = count_robots(&quad_b_start, &quad_b_end, &final_positions);
    let q3 = count_robots(&quad_c_start, &quad_c_end, &final_positions);
    let q4 = count_robots(&midpoint, &quad_d_end, &final_positions);

    let result: i64 = q1 * q2 * q3 * q4;
    result
}

fn draw_robots(map_dim: &Position, robots: &Vec<Position>) -> () {
    let mut result = Vec::new();

    for _ in 0..map_dim.x {
        let mut vec: Vec<char> = Vec::new();
        for _ in 0..map_dim.y {
            vec.push('.');
        }
        result.push(vec);
    }

    for robot in robots {
        result[robot.x as usize][robot.y as usize] = 'x';
    }

    for x in 0..map_dim.x {
        for y in 0..map_dim.y {
            print!("{}", result[x as usize][y as usize]);
        }
        print!("\n");
    }
}

fn find_run(robots: &Vec<Position>, overrun: i64) -> bool {
    assert!(robots.len() > 1);

    let mut i = 1;
    while i < robots.len() {
        let mut robot_prev = robots[i - 1];
        let mut robot_curr = robots[i];
        let mut run = 1;
        while (i < robots.len())
            && (robot_prev.x == robot_curr.x)
            && (robot_curr.y - robot_prev.y == 1)
        {
            robot_prev = robots[i - 1];
            robot_curr = robots[i];
            run += 1;
            i += 1;
        }

        if run >= overrun {
            println!("{robot_prev:?}|{run}");
            return true;
        }

        if run == 1 {
            i += 1;
        }
    }

    false
}

pub fn part2(data: &str) -> i64 {
    let robots = parse(data);
    let map_dim = Position { x: 101, y: 103 };

    for i in 0..=10000 {
        let mut positions: Vec<Position> = robots
            .iter()
            .map(|robot| advance_robot(&robot, &map_dim, i))
            .collect();
        positions.sort();

        if i % 10000 == 0 {
            println!("Iteration {i}");
        }

        if find_run(&positions, 10) {
            println!("---------------------------------------------------");
            draw_robots(&map_dim, &positions);
            println!("Iteration {i}!");
            println!("---------------------------------------------------");
            break;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part1(data), 21);
    }

    #[test]
    fn part1_works_on_midpoint() {
        let data = "p=0,2 v=1,1";
        assert_eq!(part1(data), 0);
    }
}
