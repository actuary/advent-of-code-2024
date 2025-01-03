enum Schematic {
    Lock(Vec<Vec<char>>),
    Key(Vec<Vec<char>>),
}

fn parse(data: &str) -> Vec<Schematic> {
    data.trim()
        .split("\n\n")
        .map(|block| {
            let rows: Vec<Vec<char>> = block
                .split("\n")
                .map(|line| line.chars().collect())
                .collect();

            if rows[0][0] == '#' {
                return Schematic::Lock(rows);
            } else {
                return Schematic::Key(rows);
            }
        })
        .collect()
}

fn calculate_heights(schematic: &Schematic) -> (i64, i64, i64, i64, i64) {
    let mut heights: Vec<i64> = Vec::with_capacity(5);

    match schematic {
        Schematic::Lock(pattern) => {
            for y in 0..pattern[0].len() {
                let mut x = 1;
                while x < pattern.len() - 1 && pattern[x][y] == '#' {
                    x += 1;
                }
                heights.push((x - 1) as i64);
            }
        }
        Schematic::Key(pattern) => {
            for y in 0..pattern[0].len() {
                let mut x = pattern.len() - 1;
                while x > 0 && pattern[x][y] == '#' {
                    x -= 1;
                }
                heights.push((pattern.len() - x - 2) as i64);
            }
        }
    }

    (heights[0], heights[1], heights[2], heights[3], heights[4])
}

fn check_lock_key(lock: &(i64, i64, i64, i64, i64), key: &(i64, i64, i64, i64, i64)) -> bool {
    let (l_a, l_b, l_c, l_d, l_e) = lock;
    let (k_a, k_b, k_c, k_d, k_e) = key;

    l_a + k_a <= 5
        && l_b + k_b <= 5
        && l_c + k_c <= 5
        && l_d + k_d <= 5
        && l_e + k_e <= 5
}

pub fn part1(data: &str) -> i64 {
    let schematics = parse(data);

    let mut locks: Vec<(i64, i64, i64, i64, i64)> = Vec::new();
    let mut keys: Vec<(i64, i64, i64, i64, i64)> = Vec::new();

    schematics.iter().for_each(|schematic| match schematic {
        Schematic::Lock(_) => locks.push(calculate_heights(schematic)),
        Schematic::Key(_) => keys.push(calculate_heights(schematic)),
    });

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if check_lock_key(&lock, &key) {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(data: &str) -> i64 {
    // no day 25 part 2.
    data.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(part1(data), 3);
    }
}
