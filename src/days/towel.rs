use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

fn parse_input(puzzle_input: &str) -> (Vec<&str>, Vec<&str>) {
    let (pieces, targets) = puzzle_input.split_once("\n\n").unwrap();
    (pieces.split(", ").collect(), targets.lines().collect())
}

fn count_builds(
    pieces: &HashSet<&str>,
    target: &str,
    max_piece_size: usize,
    mem: &mut HashMap<String, u64>,
) -> u64 {
    if target.is_empty() {
        return 1;
    }

    if let Some(count) = mem.get(target) {
        return *count;
    }

    let mut target_builds = 0;
    for i in 1..(max_piece_size + 1).min(target.len() + 1) {
        if pieces.contains(&target[..i]) {
            let num_suffix_builds = count_builds(pieces, &target[i..], max_piece_size, mem);

            target_builds += num_suffix_builds
        }
    }

    mem.insert(target.to_string(), target_builds);
    target_builds
}

pub struct TowelSolution;

impl Solution for TowelSolution {
    fn part1(puzzle_input: String) -> String {
        let (pieces, targets) = parse_input(&puzzle_input);
        let piece_set: HashSet<_> = pieces.iter().cloned().collect();

        let max_piece_size = pieces.iter().cloned().map(str::len).max().unwrap();

        targets
            .iter()
            .cloned()
            .filter(|target| {
                count_builds(&piece_set, *target, max_piece_size, &mut HashMap::new()) > 0
            })
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (pieces, targets) = parse_input(&puzzle_input);
        let piece_set: HashSet<_> = pieces.iter().cloned().collect();

        let max_piece_size = pieces.iter().cloned().map(str::len).max().unwrap();

        targets
            .iter()
            .cloned()
            .map(|target| count_builds(&piece_set, target, max_piece_size, &mut HashMap::new()))
            .sum::<u64>()
            .to_string()
    }
}
