use std::collections::HashMap;

use crate::solution::Solution;

pub struct PebbleCounterSolution;

fn blink_pebble(pebble: u64) -> (u64, Option<u64>) {
    match pebble {
        0 => (1, None),
        x if (x.ilog10() + 1) % 2 == 0 => {
            let num_digits = pebble.ilog10() + 1;
            let factor = u64::pow(10, num_digits / 2);

            let lhs = pebble / factor;
            let rhs = pebble - lhs * factor;

            (lhs, Some(rhs))
        }
        _ => (pebble * 2024, None),
    }
}

/// Returns the number of pubbles after a certain number of blinks
fn count_pebbles_memoized(
    pebble: u64,
    blinks: u64,
    num_pebbles: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    // Base cases
    if blinks == 0 {
        // Doesn't matter the number
        return 1;
    }

    // Lookup memoized value
    if let Some(x) = num_pebbles.get(&(pebble, blinks)) {
        return *x;
    }

    // Compute memoized value
    let (lhs, rhs) = blink_pebble(pebble);
    let result = count_pebbles_memoized(lhs, blinks - 1, num_pebbles)
        + rhs.map_or(0, |rhs| {
            count_pebbles_memoized(rhs, blinks - 1, num_pebbles)
        });

    // Memoize and return
    num_pebbles.insert((pebble, blinks), result);
    result
}

impl Solution for PebbleCounterSolution {
    fn part1(puzzle_input: String) -> String {
        let nums: Vec<u64> = puzzle_input
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let mut memtable = HashMap::new();

        nums.iter()
            .map(|n| count_pebbles_memoized(*n, 25, &mut memtable))
            .sum::<u64>()
            .to_string()
    }
    fn part2(puzzle_input: String) -> String {
        let nums: Vec<u64> = puzzle_input
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let mut memtable = HashMap::new();

        nums.iter()
            .map(|n| count_pebbles_memoized(*n, 75, &mut memtable))
            .sum::<u64>()
            .to_string()
    }
}
