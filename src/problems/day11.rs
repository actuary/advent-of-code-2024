use std::collections::HashMap;

fn parse(data: &str) -> HashMap<i64, i64> {
    data.trim()
        .split(" ")
        .map(|v| (v.parse().unwrap(), 1))
        .collect()
}

fn blink(stones: &mut HashMap<i64, i64>) {
    // we can't borrow stones immutably and also borrow mutate, 
    // so we'll collect the hits, then apply to stones counts.
    let mut hits: HashMap<i64, i64> = HashMap::new();

    for (&stone, &stone_count) in stones.iter() {
        // First, decrement the stone
        hits.insert(stone, *hits.get(&stone).unwrap_or(&0) - stone_count);

        if stone == 0 {
            // case 1:
            hits.insert(1, *hits.get(&1).unwrap_or(&0) + stone_count);
        } else {

            let number_of_digits = (stone).ilog10() + 1;

            if number_of_digits % 2 == 0 {
                // case 2:
                let top_half = stone / (10 as i64).pow(number_of_digits / 2);
                hits.insert(top_half, *hits.get(&top_half).unwrap_or(&0) + stone_count);

                let bottom_half = stone - (top_half * (10 as i64).pow(number_of_digits / 2));
                hits.insert(bottom_half, *hits.get(&bottom_half).unwrap_or(&0) + stone_count);
            } else {
                // case 3:
                let new_value = stone * 2024;
                hits.insert(new_value, *hits.get(&new_value).unwrap_or(&0) + stone_count);
            }
        }
    }

    for (stone, count) in &hits {
        let current_value = *stones.get(stone).unwrap_or(&0);
        if current_value + count == 0 {
            stones.remove(stone);
        } else {
            stones.insert(*stone, current_value + count);
        }
    }
}

pub fn part1(data: &str) -> i64 {
    let mut stones = parse(data);

    let number_of_blinks = 25;

    for _ in 0..number_of_blinks {
        blink(&mut stones);
    }

    stones.values().sum()
}

pub fn part2(data: &str) -> i64 {
    let mut stones = parse(data);

    let number_of_blinks = 75;

    for _ in 0..number_of_blinks {
        blink(&mut stones);
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn part1_works() {
    //    let data = "0 1 10 99 999";
    //    assert_eq!(part1(data), 7);
    //}

    #[test]
    fn part1_works_larger_example() {
        let data = "125 17";
        //assert_eq!(part1(data), 22);
        assert_eq!(part1(data), 55312);
    }

    #[test]
    fn part2_works() {
        let data = "";
        assert_eq!(part2(data), 0);
    }
}
