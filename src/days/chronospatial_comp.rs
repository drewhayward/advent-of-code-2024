use core::panic;
use std::{u32, u64};

use itertools::Itertools;
use regex::Regex;

use crate::{solution::Solution, utils::wait_for_input};

#[derive(Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Op {
    fn new(code: u8) -> Op {
        match code {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => panic!("invalid op code"),
        }
    }
}

#[derive(Debug)]
enum ComboOperand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl ComboOperand {
    fn new(code: u8) -> ComboOperand {
        match code {
            x if x <= 3 => ComboOperand::Literal(x),
            4 => ComboOperand::RegisterA,
            5 => ComboOperand::RegisterB,
            6 => ComboOperand::RegisterC,
            _ => panic!("invalid combo operand"),
        }
    }

    fn resolve(&self, state: &State) -> u64 {
        match self {
            ComboOperand::Literal(x) => *x as u64,
            ComboOperand::RegisterA => state.a,
            ComboOperand::RegisterB => state.b,
            ComboOperand::RegisterC => state.c,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
}

type Program = Vec<u8>;

fn parse_input(puzzle_input: String) -> (State, Program) {
    let a_pat = Regex::new(r"Register A: (\d+)").unwrap();
    let b_pat = Regex::new(r"Register B: (\d+)").unwrap();
    let c_pat = Regex::new(r"Register C: (\d+)").unwrap();

    // Lol, I hate this
    let state = State {
        a: a_pat
            .captures(&puzzle_input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        b: b_pat
            .captures(&puzzle_input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        c: c_pat
            .captures(&puzzle_input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
    };

    let (_, program_string) = puzzle_input.split_once("\n\n").unwrap();

    let program: Program = program_string[9..]
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    return (state, program);
}

fn run(program: &Program, state: &State, quine: bool) -> (Vec<u8>, State) {
    let mut current_state = state.clone();
    let mut ip: usize = 0;
    let mut output = Vec::new();

    while let Some(op_code) = program.get(ip).cloned() {
        let op = Op::new(op_code);

        match op {
            Op::Adv => {
                let operand =
                    ComboOperand::new(*program.get(ip + 1).expect("Operand should exist"));
                let num = current_state.a;
                let denom = (2 as u64).pow(operand.resolve(&current_state) as u32);
                current_state.a = num / denom;
            }
            Op::Bxl => {
                let operand = *program.get(ip + 1).expect("Operand should exist");
                let value = current_state.b;

                current_state.b = value ^ operand as u64;
            }
            Op::Bst => {
                let operand =
                    ComboOperand::new(*program.get(ip + 1).expect("Operand should exist"));
                current_state.b = operand.resolve(&current_state).rem_euclid(8);
            }
            Op::Jnz => {
                let operand = *program.get(ip + 1).expect("Operand should exist");
                if current_state.a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            Op::Bxc => {
                current_state.b = current_state.b ^ current_state.c;
            }
            Op::Out => {
                let operand =
                    ComboOperand::new(*program.get(ip + 1).expect("Operand should exist"));
                let value = operand.resolve(&current_state).rem_euclid(8) as u8;
                output.push(value);
            }
            Op::Bdv => {
                let operand =
                    ComboOperand::new(*program.get(ip + 1).expect("Operand should exist"));
                let num = current_state.a;
                let denom = (2 as u64).pow(operand.resolve(&current_state) as u32);
                current_state.b = num / denom;
            }
            Op::Cdv => {
                let operand =
                    ComboOperand::new(*program.get(ip + 1).expect("Operand should exist"));
                let num = current_state.a;
                let denom = (2 as u64).pow(operand.resolve(&current_state) as u32);
                current_state.c = num / denom;
            }
        }

        ip += 2;

        // Check for exit condition
        if quine {
            for (prog, out) in program.iter().zip(&output) {
                if prog != out {
                    return (output, current_state);
                }
            }
        }
    }

    (output, current_state)
}

/// Recursively works backwards to generate a number for part 2
fn generate_num(program: &Program, state: &State, seed: u64, targets: &[u8]) -> Option<u64> {
    let target_value = targets.first().unwrap();

    // For each 3 byte num
    for i in 0..8 {
        // Try it out in a clean substate
        let mut sub_state = state.clone();
        // Slide the existing answer over by 3 to make room
        let candidate = (seed * (2 as u64).pow(3)) + i;
        sub_state.a = candidate;
        // Run the program and check the output
        let (output, _) = run(&program, &sub_state, true);

        // If we generated the right walue
        if output.first().unwrap() == target_value {
            // If we have remaining digits to generate
            if targets.len() > 1 {
                // Recurse and try to generate the remaining digits
                // This may fail because the generated bit is affected by the
                // lowest 7 bits of the input number 
                let recursive_result = generate_num(program, state, candidate, &targets[1..]);

                if recursive_result.is_some() {
                    return recursive_result
                }

            } else {
                // Yay! it worked
                return Some(candidate)
            }
        }
    }

    // If it never worked, return none and stop exploring this bit prefix
    None
}

pub struct ChronospatialSolution;

impl Solution for ChronospatialSolution {
    fn part1(puzzle_input: String) -> String {
        let (state, program) = parse_input(puzzle_input);

        let (output, _) = run(&program, &state, false);

        output.iter().join(",")
    }

    fn part2(puzzle_input: String) -> String {
        let (state, program) = parse_input(puzzle_input);
        //let mut value = None;

        // This works in general, but will take eons
        //let start: u64 = (2 as u64).pow(48);
        //let end: u64 = (2 as u64).pow(49) - 1;
        //let value = (start..end).into_par_iter().find_map_first(|i| {
        //    let mut sub_state = state.clone();
        //    sub_state.a = i;
        //    let (output, end_state) = run(&program, &sub_state, true);
        //
        //    if output.len() > 12 {
        //        println!("{} {:?} {:?}", sub_state.a, output, end_state);
        //    }
        //
        //    if output == program {
        //        Some(i)
        //    } else {
        //        None
        //    }
        //});

        let target: Vec<_> = program.iter().rev().cloned().collect();

        let result = generate_num(&program, &state, 0, &target).unwrap();

        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let state = State { a: 0, b: 0, c: 9 };
        let program: Program = vec![2, 6];

        let (_, out_state) = run(&program, &state, false);
        assert_eq!(out_state.b, 1);
    }

    #[test]
    fn test_case_2() {
        let state = State { a: 10, b: 0, c: 0 };
        let program: Program = vec![5, 0, 5, 1, 5, 4];
        let expected_output = vec![0, 1, 2];

        let (output, _) = run(&program, &state, false);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_case_3() {
        let state = State {
            a: 2024,
            b: 0,
            c: 0,
        };
        let program: Program = vec![0, 1, 5, 4, 3, 0];
        let expected_output = vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0];

        let (output, out_state) = run(&program, &state, false);
        assert_eq!(out_state.a, 0);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_case_4() {
        let state = State { a: 0, b: 29, c: 0 };
        let program: Program = vec![1, 7];

        let (_, out_state) = run(&program, &state, false);
        assert_eq!(out_state.b, 26);
    }

    #[test]
    fn test_case_5() {
        let state = State {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let program: Program = vec![4, 0];

        let (_, out_state) = run(&program, &state, false);
        assert_eq!(out_state.b, 44354);
    }
}
