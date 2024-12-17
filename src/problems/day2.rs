fn parse(data: &str) -> Vec<Vec<i32>> {
    let lines: Vec<Vec<i32>> = data
        .trim()
        .split("\n")
        .map(|line| line.split(" ").map(|val| val.parse().unwrap()).collect())
        .collect();

    lines
}

fn is_safe_value(a: u32) -> bool {
    (0 < a) && (a <= 3)
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() <= 2 {
        // of course a 1 or 2 items report is safe
        return true;
    }

    let initial_direction = report[0].cmp(&report[1]);

    match initial_direction {
        std::cmp::Ordering::Equal => return false,
        _ => {
            for window in report[..].windows(2) {
                let direction = window[0].cmp(&window[1]);

                if initial_direction != direction || !is_safe_value(window[0].abs_diff(window[1])) {
                    return false;
                }
            }

            true
        }
    }
}

fn is_nearly_safe(report: &[i32]) -> bool {
    if is_safe(&report[1..]) || is_safe(&report[..report.len() - 1]) {
        // if it's safe then 1 shorter at either end is safe.
        // handles the size 3
        return true;
    }

    // it must be at least 4 long.
    // first and last value must be ok, due to condition above.
    // so any error must be somewhere in the middle.
    // we want to know if it's increasing or decreasing now, and we want
    // this to be consistent. 
    // If it is nearly safe now, first and last are correct, and we 
    // compare all ordering to these.
    let order = report[0].cmp(report.last().unwrap());

    let mut unsafes = 0;
    let mut last_safe = report[0];
    for &value in &report[1..] {
        if !is_safe_value(last_safe.abs_diff(value)) {
            unsafes += 1;
        } else if last_safe.cmp(&value) != order {
            unsafes += 1;
        } else {
            last_safe = value;
        }
    }

    assert!(unsafes > 0);
    unsafes == 1
}

pub fn part1(data: &str) -> i32 {
    let reports = parse(data);

    let result: i32 = reports.iter().map(|report| is_safe(report) as i32).sum();
    result
}

pub fn part2(data: &str) -> i32 {
    let reports = parse(data);

    let result: i32 = reports
        .iter()
        .map(|report| is_nearly_safe(&report[..]) as i32)
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(data), 2);
    }

    #[test]
    fn part2_works() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(data), 4);
    }
}
