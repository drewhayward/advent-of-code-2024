use core::panic;
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
enum Map {
    Wall,
    Space,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_point(&self) -> Point {
        match self {
            Direction::Up => Point(0, -1),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn parse_input(puzzle_input: String) -> (HashMap<Point, Map>, Point) {
    let mut hm = HashMap::new();
    let mut pos = None;
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            hm.insert(
                Point(x as i32, y as i32),
                match cell {
                    '#' => Map::Wall,
                    '.' => Map::Space,
                    '^' => {
                        pos = Some(Point(x as i32, y as i32));
                        Map::Space
                    }
                    x => panic!("Unknown map element {x}"),
                },
            );
        }
    }

    (hm, pos.expect("Map should have starting point"))
}

pub struct GuardSolution;

impl GuardSolution {
    fn add_obstruction_and_check_for_cycle(
        obstruction: &Point,
        mut map: HashMap<Point, Map>,
        state: (Point, Direction),
    ) -> bool {
        let mut visited = HashSet::new();
        let mut current_state = state;

        // Add the new obstruction
        map.insert(obstruction.clone(), Map::Wall);

        loop {
            let (pos, dir) = current_state;

            // Check for looping
            if visited.contains(&current_state) {
                return true;
            }

            visited.insert(current_state);
            current_state = match map.get(&(pos + dir.as_point())) {
                Some(Map::Wall) => (pos, dir.rotate()),
                Some(Map::Space) => (pos + dir.as_point(), dir),
                None => return false,
            }
        }
    }
}

impl Solution for GuardSolution {
    fn part1(puzzle_input: String) -> String {
        let (map, start_pos) = parse_input(puzzle_input);

        let mut visited = HashSet::new();
        let mut current_state = (start_pos, Direction::Up);
        loop {
            let (pos, dir) = current_state;
            visited.insert(pos);
            current_state = match map.get(&(pos + dir.as_point())) {
                Some(Map::Wall) => (pos, dir.rotate()),
                Some(Map::Space) => (pos + dir.as_point(), dir),
                None => break,
            }
        }

        visited.len().to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (map, start_pos) = parse_input(puzzle_input);

        // Collected the candidate locations for the obstacle
        let mut visited = HashSet::new();
        let mut current_state = (start_pos, Direction::Up);
        loop {
            let (pos, dir) = current_state;
            visited.insert(pos);

            // Advance state
            current_state = match map.get(&(pos + dir.as_point())) {
                Some(Map::Wall) => (pos, dir.rotate()),
                Some(Map::Space) => (pos + dir.as_point(), dir),
                None => break,
            }
        }

        // Remove the non-allowed positions
        visited.remove(&(start_pos + Direction::Up.as_point()));
        visited.remove(&(start_pos));

        visited
            .iter()
            .filter_map(|v| {
                GuardSolution::add_obstruction_and_check_for_cycle(
                    v,
                    map.clone(),
                    (start_pos, Direction::Up),
                )
                .then_some(1)
            })
            .count()
            .to_string()
    }
}
