use core::panic;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use cached::proc_macro::cached;

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn get_numpad() -> HashMap<char, Point> {
    ([
        ('7', Point(0, 0)),
        ('8', Point(1, 0)),
        ('9', Point(2, 0)),
        ('4', Point(0, 1)),
        ('5', Point(1, 1)),
        ('6', Point(2, 1)),
        ('1', Point(0, 2)),
        ('2', Point(1, 2)),
        ('3', Point(2, 2)),
        ('0', Point(1, 3)),
        ('A', Point(2, 3)),
    ])
    .iter()
    .cloned()
    .collect()
}

fn get_dirpad() -> HashMap<char, Point> {
    ([
        ('^', Point(1, 3)),
        ('A', Point(2, 3)),
        ('<', Point(0, 4)),
        ('v', Point(1, 4)),
        ('>', Point(2, 4)),
    ])
    .iter()
    .cloned()
    .collect()
}

fn movement_to_instructions(movement: Point) -> Vec<String> {
    let horizontal_moves = if movement.0 > 0 {
        std::iter::repeat(">")
            .take(movement.0.abs() as usize)
            .collect::<String>()
    } else if movement.0 < 0 {
        std::iter::repeat("<")
            .take(movement.0.abs() as usize)
            .collect::<String>()
    } else {
        String::new()
    };
    let vertical_moves = if movement.1 > 0 {
        std::iter::repeat("v")
            .take(movement.1.abs() as usize)
            .collect::<String>()
    } else if movement.1 < 0 {
        std::iter::repeat("^")
            .take(movement.1.abs() as usize)
            .collect::<String>()
    } else {
        String::new()
    };

    vec![
        vertical_moves.clone() + &horizontal_moves + "A",
        horizontal_moves + &vertical_moves + "A",
    ]
}

fn dir_to_point(dir: char) -> Point {
    match dir {
        '^' => Point(0, -1),
        '>' => Point(1, 0),
        'v' => Point(0, 1),
        '<' => Point(-1, 0),
        'A' => Point(0, 0),
        _ => panic!(),
    }
}

#[cached]
fn min_steps(
    direction_key: char,
    prev: Option<char>,
    dirpad_indirects: u64,
    use_numpad: bool,
) -> u64 {
    let keypad = if use_numpad {
        get_numpad()
    } else {
        get_dirpad()
    };
    let start_pos = keypad.get(&prev.unwrap_or('A')).unwrap();
    let target_position = keypad.get(&direction_key).unwrap();
    if dirpad_indirects == 0 {
        let directions = movement_to_instructions(*target_position - *start_pos);
        return directions[0].len() as u64;
    }

    // Look at each possible way of outputting this key and choose the lowest possible one
    let mut lowest_steps: Option<u64> = None;
    'outer: for directions in movement_to_instructions(*target_position - *start_pos) {
        let mut steps = 0;
        let mut pos = start_pos.clone();
        let mut dir_prev = None;
        for direction_key in directions.chars() {
            // if we cross over the death zone, abandon this path
            pos = pos + dir_to_point(direction_key);
            if pos == Point(0, 3) {
                continue 'outer;
            }
            steps += min_steps(direction_key, dir_prev, dirpad_indirects - 1, false);

            dir_prev = Some(direction_key);
        }

        lowest_steps = lowest_steps.map_or(Some(steps), |lowest| Some(lowest.min(steps)));
    }

    lowest_steps.unwrap()
}

fn min_code_moves(code: &str, dirpad_indirects: u64) -> u64 {
    let mut prev = None;
    let mut num_directions = 0;
    for c in code.chars() {
        num_directions += min_steps(c, prev, dirpad_indirects, true);
        prev = Some(c)
    }

    num_directions
}

pub struct Keypad;

impl Solution for Keypad {
    fn part1(puzzle_input: String) -> String {
        puzzle_input
            .lines()
            .map(|code| {
                let code_num: u64 = code[..code.len() - 1].parse().unwrap();
                min_code_moves(&code, 2) * code_num
            })
            .sum::<u64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        puzzle_input
            .lines()
            .map(|code| {
                let code_num: u64 = code[..code.len() - 1].parse().unwrap();
                min_code_moves(&code, 25) * code_num
            })
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_depth_1() {
        assert_eq!(min_steps('3', None, 0, true), 2);
        assert_eq!(min_steps('2', None, 0, true), 3);
        assert_eq!(min_steps('7', None, 0, true), 6);
        assert_eq!(min_steps('A', Some('A'), 0, true), 1);
        assert_eq!(min_steps('7', Some('7'), 0, true), 1);
    }
}
