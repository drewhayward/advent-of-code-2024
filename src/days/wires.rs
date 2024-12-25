use core::panic;
use std::collections::HashMap;

use crate::solution::Solution;

#[derive(Debug, Clone)]
enum GateOp {
    And,
    Or,
    Xor,
}

impl GateOp {
    fn eval(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            GateOp::And => lhs && rhs,
            GateOp::Or => lhs || rhs,
            GateOp::Xor => lhs ^ rhs,
        }
    }
}

#[derive(Debug, Clone)]
struct Gate<'circuit> {
    op: GateOp,
    lhs: &'circuit str,
    rhs: &'circuit str,
    output: &'circuit str,
}

type CircuitState = HashMap<String, bool>;

fn parse_input<'a>(puzzle_input: &'a str) -> (CircuitState, Vec<Gate<'a>>) {
    let (inputs, gates) = puzzle_input.split_once("\n\n").unwrap();

    let inputs = inputs.lines().map(|line| {
        let (var, value) = line.split_once(": ").unwrap();
        (var.to_string(), value == "1")
    });

    let gates = gates.lines().map(|line| {
        let (operation, output) = line.split_once(" -> ").unwrap();
        let operation: Vec<_> = operation.split(" ").collect();

        Gate {
            lhs: operation[0],
            op: match operation[1] {
                "AND" => GateOp::And,
                "XOR" => GateOp::Xor,
                "OR" => GateOp::Or,
                _ => panic!(),
            },
            rhs: operation[2],
            output,
        }
    });

    (inputs.collect(), gates.collect())
}

fn step<'circuit>(state: &mut CircuitState, gates: &'circuit [Gate]) -> bool {
    let mut next_state = state.clone();
    let mut modified = false;
    for gate in gates {
        match (state.get(gate.lhs), state.get(gate.rhs)) {
            (Some(lhs), Some(rhs)) => {
                let new_value = gate.op.eval(*lhs, *rhs);

                if let Some(old_value) = state.get(gate.output) {
                    modified = modified || (new_value != *old_value);
                }
                next_state.insert(gate.output.to_string(), new_value);
            }
            _ => {}
        }
    }

    std::mem::swap(state, &mut next_state);
    modified
}

fn all_zvalues_set(state: &CircuitState, zvalues: &[&str]) -> bool {
    zvalues.iter().all(|value| state.get(*value).is_some())
}

fn read_zvalues(state: &CircuitState) -> u64 {
    let mut zvalues: Vec<_> = state
        .keys()
        .cloned()
        .filter(|key| key.starts_with('z'))
        .collect();
    zvalues.sort();

    let mut output = 0;
    for zvalue in zvalues.iter().rev() {
        output = (output << 1) + (*state.get(zvalue).unwrap() as u64);
    }

    output
}

fn set_state(variable: char, value: u64, state: &mut CircuitState) {
    for i in 0..45 {
        let wire = format!("{variable}{i:0>2}");
        let wire_value = ((value >> i) & 1) == 1;

        if let Some(value) = state.get_mut(&wire) {
            *value = wire_value;
        }
    }
}

fn run_circuit(x: u64, y: u64, gates: &[Gate]) -> u64 {
    let mut circuit_state: CircuitState = HashMap::new();

    let zvalues: Vec<_> = gates
        .iter()
        .cloned()
        .filter_map(|gate| gate.output.starts_with('z').then_some(gate.output))
        .collect();

    set_state('x', x, &mut circuit_state);
    set_state('y', y, &mut circuit_state);

    while !all_zvalues_set(&circuit_state, &zvalues) {
        step(&mut circuit_state, &gates);
    }

    read_zvalues(&circuit_state)
}

pub struct CrossedWires;

impl Solution for CrossedWires {
    fn part1(puzzle_input: String) -> String {
        let (inputs, gates) = parse_input(&puzzle_input);

        let zvalues: Vec<_> = gates
            .iter()
            .cloned()
            .filter_map(|gate| gate.output.starts_with('z').then_some(gate.output))
            .collect();

        let mut circuit_state = inputs.clone();
        while !all_zvalues_set(&circuit_state, &zvalues) {
            step(&mut circuit_state, &gates);
        }

        read_zvalues(&circuit_state).to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (inputs, gates) = parse_input(&puzzle_input);

        let zvalues: Vec<_> = gates
            .iter()
            .cloned()
            .filter_map(|gate| gate.output.starts_with('z').then_some(gate.output))
            .collect();

        let mut circuit_state = inputs.clone();
        while !all_zvalues_set(&circuit_state, &zvalues) {
            step(&mut circuit_state, &gates);
        }

        // Sample N addition examples and run them through the circuit
        // Any bits which are ever incorrect are being affected by flipped gates
        let x = 1;
        let y = 2;

        // TODO: Do I need to mask out bits beyond the 45th in case of carry?
        let expected = x + y;

        // Create two sets, the set of bits which are sometimes incorrect and another of the always
        // correct bits.

        // Search backwards through the circuit to find the "reachable" set of gates for each set.
        // Assume gates which are only reachable from bad bits are the candidate flipped gates.

        // ... Profit? (Depending on how many gates are candidate bad gates we may be able to just
        // search through that space)
        read_zvalues(&circuit_state).to_string()
    }
}
