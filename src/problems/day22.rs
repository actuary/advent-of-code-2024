use std::collections::HashMap;

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn parse(data: &str) -> Vec<i64> {
    data.trim()
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect()
}

fn next_secret_number(secret_number: i64, times: usize) -> Vec<i64> {
    let generator = compose!(
        |n| ((n * 64) ^ n) % 16777216,
        |n| ((n / 32) ^ n) % 16777216,
        |n| ((n * 2048) ^ n) % 16777216
    );

    let mut result = vec![secret_number];

    (0..times).fold(secret_number, |n, _| {
        let new_number = generator(n);
        result.push(new_number);
        new_number
    });

    result
}

pub fn part1(data: &str) -> i64 {
    let buyers = parse(data);

    buyers
        .iter()
        .map(|&num| *next_secret_number(num, 2000).last().unwrap())
        .sum()
}

pub fn part2(data: &str) -> i64 {
    let buyers = parse(data);
    let iterations = 2000;

    let secret_numbers: Vec<Vec<i64>> = buyers
        .iter()
        .map(|&num| next_secret_number(num, iterations))
        .collect();
    let digits: Vec<Vec<i64>> = secret_numbers
        .iter()
        .map(|numbers| numbers.iter().map(|n| n % 10).collect())
        .collect();
    let deltas: Vec<Vec<i64>> = digits
        .iter()
        .map(|numbers| {
            numbers
                .windows(2)
                .map(|slice| slice[1] - slice[0])
                .collect()
        })
        .collect();

    let mut best_prices: HashMap<i64, HashMap<(i64, i64, i64, i64), i64>> = HashMap::new();

    for ((delta_per_buyer, digit_per_buyer), buyer_number) in deltas.iter().zip(digits).zip(buyers)
    {
        let mut buyer_best: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        for (m, delta_window) in delta_per_buyer.windows(4).enumerate() {
            let (i, j, k, l) = (
                delta_window[0],
                delta_window[1],
                delta_window[2],
                delta_window[3],
            );

            if !buyer_best.contains_key(&(i, j, k, l)) {
                buyer_best.insert((i, j, k, l), digit_per_buyer[m + 4]);
            }
        }

        best_prices.insert(buyer_number, buyer_best);
    }

    let mut final_best_prices = HashMap::new();
    for (_, buyer_best_prices) in best_prices.iter() {
        for (k, v) in buyer_best_prices.iter() {
            match final_best_prices.get(k) {
                Some(current_value) => {
                    final_best_prices.insert(*k, current_value + *v);
                }
                None => {
                    final_best_prices.insert(*k, *v);
                }
            }
        }
    }

    *final_best_prices.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "1
10
100
2024";
        assert_eq!(part1(data), 37327623);
    }

    #[test]
    fn part2_simple() {
        let data = "123";
        assert_eq!(part2(data), 9);
    }

    #[test]
    fn part2_works() {
        let data = "1
2
3
2024";
        assert_eq!(part2(data), 23);
    }
}
