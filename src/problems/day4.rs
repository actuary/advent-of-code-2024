fn parse(data: &str) -> Vec<Vec<char>> {
    data.trim().split("\n")
        .map(|line| line.as_bytes().iter().map(|c| *c as char).collect())
        .collect()
}

static DIRECTIONS: &'static [(i32, i32)] = &[
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn count_word(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &[char]) -> u32 {
    let mut result: u32 = 0;
    for (i, j) in DIRECTIONS.iter() {
        let mut matches = true;

        for letter_idx in 0..=3 {
            let end_i = start_i as i32 + (i * letter_idx);
            let end_j = start_j as i32 + (j * letter_idx);

            if (end_i >= 0)
                && (end_i < grid.len() as i32)
                && (end_j >= 0)
                && (end_j < grid[0].len() as i32)
            {
                let value = grid[end_i as usize][end_j as usize];
                matches &= value == word[letter_idx as usize];
            } else {
                matches = false;
            }
        }
        result += matches as u32;
    }

    result
}

fn count_x_word(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize) -> u32 {
    assert!(start_i > 0 && start_i < grid.len() - 1);
    assert!(start_j > 0 && start_j < grid[0].len() - 1);
    assert!(grid[start_i][start_j]=='A');

    let result = (
        (grid[start_i - 1][start_j + 1] == 'M' && grid[start_i + 1][start_j - 1] == 'S') &&
        (grid[start_i - 1][start_j - 1] == 'M' && grid[start_i + 1][start_j + 1] == 'S')
    ) || (
        (grid[start_i - 1][start_j + 1] == 'S' && grid[start_i + 1][start_j - 1] == 'M') &&
        (grid[start_i - 1][start_j - 1] == 'M' && grid[start_i + 1][start_j + 1] == 'S')
    ) || (
        (grid[start_i - 1][start_j + 1] == 'M' && grid[start_i + 1][start_j - 1] == 'S') &&
        (grid[start_i - 1][start_j - 1] == 'S' && grid[start_i + 1][start_j + 1] == 'M')
    ) || (
        (grid[start_i - 1][start_j + 1] == 'S' && grid[start_i + 1][start_j - 1] == 'M') &&
        (grid[start_i - 1][start_j - 1] == 'S' && grid[start_i + 1][start_j + 1] == 'M')
    );

    result as u32
}

pub fn part1(data: &str) -> u32 {
    let mut result: u32 = 0;

    let grid = parse(data);

    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 'X' {
                result += count_word(&grid, i, j, &['X', 'M', 'A', 'S']);
            }
        }
    }

    result
}

pub fn part2(data: &str) -> u32 {
    let mut result: u32 = 0;

    let grid = parse(data);

    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if (i > 0 && i < grid.len() - 1) && value == 'A' && (j > 0 && j < grid[0].len() - 1) {
                result += count_x_word(&grid, i, j);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(data), 18);
    }

    #[test]
    fn part2_works() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(data), 9);
    }
}
