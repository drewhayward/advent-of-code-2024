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
    fn is_valid(target: u64, eqn: Equation, use_concat: bool) -> bool {
        if eqn.result > target {
            return false;
        }

        if eqn.terms.is_empty() {
            return eqn.result == target;
        }

        let mut result = Equation::is_valid(
            target,
            Equation {
                result: eqn.result + eqn.terms[0],
                terms: eqn.terms[1..].iter().cloned().collect(),
            },
            use_concat,
        ) || Equation::is_valid(
            target,
            Equation {
                result: eqn.result * eqn.terms[0],
                terms: eqn.terms[1..].iter().cloned().collect(),
            },
            use_concat,
        );

        if use_concat {
            result = result
                || Equation::is_valid(
                    target,
                    Equation {
                        result: concat(eqn.result, eqn.terms[0]),
                        terms: eqn.terms[1..].iter().cloned().collect(),
                    },
                    use_concat,
                );
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
            .filter_map(|e| {
                Equation::is_valid(
                    e.result,
                    Equation {
                        result: 0,
                        terms: e.terms.clone(),
                    },
                    false
                )
                .then_some(e.result)
            })
            .sum::<u64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let eqns = parse_input(puzzle_input);

        eqns.iter()
            .filter_map(|e| {
                Equation::is_valid(
                    e.result,
                    Equation {
                        result: e.terms[0],
                        terms: e.terms[1..].iter().cloned().collect(),
                    },
                    true
                )
                .then_some(e.result)
            })
            .sum::<u64>()
            .to_string()
    }
}
