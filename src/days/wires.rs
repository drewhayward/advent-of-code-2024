use core::panic;
use std::collections::{HashMap, HashSet};

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

fn read_value(var: char, state: &CircuitState) -> u64 {
    let mut wire_values: Vec<_> = state
        .keys()
        .cloned()
        .filter(|key| key.starts_with(var))
        .collect();
    wire_values.sort();

    let mut output = 0;
    for zvalue in wire_values.iter().rev() {
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

fn run_circuit(x: u64, y: u64, max_steps: u64, gates: &[Gate]) -> u64 {
    let mut circuit_state: CircuitState = HashMap::new();

    let zvalues: Vec<_> = gates
        .iter()
        .cloned()
        .filter_map(|gate| gate.output.starts_with('z').then_some(gate.output))
        .collect();

    set_state('x', x, &mut circuit_state);
    set_state('y', y, &mut circuit_state);

    let mut num_steps = 0;
    while !all_zvalues_set(&circuit_state, &zvalues) && num_steps < max_steps {
        step(&mut circuit_state, &gates);
        num_steps += 1
    }

    read_value('z', &circuit_state)
}

/// Starting from wire, search backwards through the gates to find all wires which affect the
/// result
fn search_gates(wire: String, gates: &[Gate], backward: bool) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut frontier = vec![wire];
    while let Some(wire) = frontier.pop() {
        if visited.contains(&wire) {
            continue;
        } else {
            visited.insert(wire.clone());
        }

        for gate in gates {
            match backward {
                true if gate.output == wire => {
                    frontier.push(gate.lhs.to_string());
                    frontier.push(gate.rhs.to_string());
                }
                false if gate.lhs == wire || gate.rhs == wire => {
                    frontier.push(gate.output.to_string().clone());
                }
                _ => {}
            }
        }
    }

    visited
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
        let mut num_steps = 0;
        while !all_zvalues_set(&circuit_state, &zvalues) {
            step(&mut circuit_state, &gates);
            num_steps += 1;
        }

        read_value('z', &circuit_state).to_string()
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
            let x = read_value('x', &circuit_state);
            let y = read_value('y', &circuit_state);
            let z = read_value('z', &circuit_state);

            println!("x: {x:#064b}");
            println!("y: {y:#064b}");
            println!("z: {z:#064b}");
            println!("+: {:#064b}", (x + y));
            println!("^: {:#064b}", z ^ (x + y));
            println!("---");
        }
        let x = read_value('x', &circuit_state);
        let y = read_value('y', &circuit_state);
        let z = read_value('z', &circuit_state);

        let expected = x + y;
        let bad_bits = expected ^ z;

        println!("The bad bit mask is {bad_bits} or {bad_bits:b}");

        // Create two sets, the set of bits which are sometimes incorrect and another of the always
        // correct bits.
        let mut bad_wires = Vec::new();
        let mut good_wires = Vec::new();
        for i in 0..45 {
            let is_bad = ((bad_bits >> i) & 1) == 1;
            let name = format!("z{i:0>2}");
            if is_bad {
                bad_wires.push(name);
            } else {
                good_wires.push(name);
            }
        }

        // Search backwards through the circuit to find the "reachable" set of gates for each set.
        // Assume gates which are only reachable from bad bits are the candidate flipped gates.
        let mut reachable_bad = HashSet::new();
        for bad_wire in bad_wires {
            let reachable = search_gates(bad_wire, &gates, true);
            reachable_bad.extend(reachable);
        }

        let mut reachable_good = HashSet::new();
        for good_wire in good_wires {
            let reachable = search_gates(good_wire, &gates, true);
            reachable_good.extend(reachable);
        }

        let mut tainted: HashSet<_> = reachable_bad.difference(&reachable_good).cloned().collect();
        dbg!(&tainted);
        dbg!(search_gates("cdk".to_string(), &gates, false));
        dbg!(search_gates("rmn".to_string(), &gates, false));

        // ... Profit? (Depending on how many gates are candidate bad gates we may be able to just
        // search through that space)
        String::new()
    }
}
