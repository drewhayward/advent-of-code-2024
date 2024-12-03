use regex::{Captures, Regex};

use crate::solution::Solution;

#[derive(Debug)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}

impl Op {
    fn new_from_capture(capture: Captures) -> Op {
        match capture[0] {
            _ if capture[0].starts_with("mul") => Op::Mul(
                capture["arg1"].parse::<i64>().unwrap(),
                capture["arg2"].parse::<i64>().unwrap(),
            ),
            _ if capture[0].starts_with("don't") => Op::Dont,
            _ => Op::Do,
        }
    }
}

pub struct MullSolution;

impl MullSolution {}

impl Solution for MullSolution {
    fn part1(puzzle_input: String) -> String {
        let re = Regex::new(r"mul\((?<arg1>\d{1,3}),(?<arg2>\d{1,3})\)").unwrap();

        return re
            .captures_iter(&puzzle_input)
            .map(|c| {
                c["arg1"].parse::<i64>().unwrap() * c["arg2"].parse::<i64>().unwrap()
            })
            .sum::<i64>()
            .to_string();
    }

    fn part2(puzzle_input: String) -> String {
        let re = Regex::new(r"mul\((?<arg1>\d{1,3}),(?<arg2>\d{1,3})\)|do\(\)|don't\(\)").unwrap();

        return re
            .captures_iter(&puzzle_input)
            .map(Op::new_from_capture)
            .fold((true, 0), |acc, op| {
                let (is_enabled, total) = acc;
                match op {
                    Op::Do if !is_enabled => (true, total),
                    Op::Dont if is_enabled => (false, total),
                    Op::Mul(a, b) if is_enabled => (true, total + a * b),
                    _ => (is_enabled, total)
                }
            })
            .1
            .to_string();
    }
}
