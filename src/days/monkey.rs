use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::solution::Solution;

const MAX_SECRET_SIZE: u64 = 16777216;

fn step_number(secret: u64) -> u64 {
    let mut result = secret;
    result = result ^ (result * 64);
    result = result.rem_euclid(MAX_SECRET_SIZE);

    result = result ^ (result / 32);
    result = result.rem_euclid(MAX_SECRET_SIZE);

    result = result ^ (result * 2048);
    result = result.rem_euclid(MAX_SECRET_SIZE);

    result
}

fn nth_secret(secret: u64, n: u64) -> u64 {
    let mut result = secret;
    for _ in 0..n {
        result = step_number(result)
    }
    result
}

type History = (i8, i8, i8, i8);

fn analyze_nums(seed: u64, look_forward: u64, global_data: &mut HashMap<History, u64>) {
    let mut secret = seed;
    let mut prev: Option<u64> = None;
    let mut price_history = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..look_forward + 1 {
        if let Some(previous_secret) = prev {
            let delta = secret.rem_euclid(10) as i8 - previous_secret.rem_euclid(10) as i8;
            price_history.push(delta);
        }

        if let Some(chunk) = price_history.last_chunk::<4>() {
            let key = (chunk[0], chunk[1], chunk[2], chunk[3]);
            let value = secret.rem_euclid(10);

            // If we've seen this history before, we won't ever get to sell it on this buyer
            if !seen.contains(&key) {
                global_data
                    .entry(key)
                    .and_modify(|num| *num += value)
                    .or_insert(value);
                seen.insert(key);
            }
        }

        prev = Some(secret);
        secret = step_number(secret);
    }

    assert_eq!(price_history.len(), look_forward as usize);
}

pub struct MonkeyMarket;

impl Solution for MonkeyMarket {
    fn part1(puzzle_input: String) -> String {
        let secret_nums = puzzle_input.lines().map(str::parse).map(Result::unwrap);

        secret_nums
            .map(|secret| nth_secret(secret, 2000))
            .sum::<u64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let secret_nums: Vec<u64> = puzzle_input
            .lines()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let mut data = HashMap::new();
        for seed in secret_nums {
            analyze_nums(seed, 2000, &mut data);
        }

        data.values().max().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mixes_correctly() {
        let mut secret = 123;
        secret = step_number(secret);
        assert_eq!(secret, 15887950);

        secret = step_number(secret);
        assert_eq!(secret, 16495136);
    }

    #[test]
    fn test_analysis_1() {
        let mut data = HashMap::new();
        analyze_nums(123, 10, &mut data);
        assert_eq!(data.get(&(-1, -1, 0, 2)), Some(&6));
        assert_eq!(data.get(&(2, -2, 0, -2)).cloned(), Some(2));
    }

    #[test]
    fn test_analysis_2() {
        let mut data = HashMap::new();
        analyze_nums(1, 2000, &mut data);
        dbg!(&data);
        assert_eq!(data.get(&(-2, 1, -1, 3)).cloned(), Some(7));
    }

    #[test]
    fn part2() {
        let input = "1
2
3
2024"
            .to_string();

        assert_eq!(MonkeyMarket::part2(input), "23");
    }
}
