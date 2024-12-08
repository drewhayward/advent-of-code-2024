use crate::solution::Solution;

fn concat(lhs: u64, rhs: u64) -> u64 {
    let size = rhs.ilog10() + 1;
    (lhs * (10 as u64).pow(size)) + rhs
}

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    terms: Vec<u64>,
}

impl Equation {
    fn is_valid(target: u64, current: u64, terms: &[u64], use_concat: bool) -> bool {
        if current > target {
            return false;
        }

        if terms.is_empty() {
            return current == target;
        }

        let mut result = Equation::is_valid(target, current + terms[0], &terms[1..], use_concat)
            || Equation::is_valid(target, current * terms[0], &terms[1..], use_concat);

        if use_concat {
            result = result
                || Equation::is_valid(target, concat(current, terms[0]), &terms[1..], use_concat);
        }

        result
    }
}

fn parse_input(puzzle_input: String) -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in puzzle_input.lines() {
        let (result, terms) = line.split_once(": ").unwrap();

        equations.push(Equation {
            result: result.parse().unwrap(),
            terms: terms.split(" ").map(|t| t.parse().unwrap()).collect(),
        })
    }

    equations
}

pub struct BridgeSolution;

impl Solution for BridgeSolution {
    fn part1(puzzle_input: String) -> String {
        let eqns = parse_input(puzzle_input);

        eqns.iter()
            .filter_map(|e| Equation::is_valid(e.result, 0, &e.terms, false).then_some(e.result))
            .sum::<u64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let eqns = parse_input(puzzle_input);

        eqns.iter()
            .filter_map(|e| Equation::is_valid(e.result, 0, &e.terms, true).then_some(e.result))
            .sum::<u64>()
            .to_string()
    }
}
