use std::{
    collections::{BinaryHeap, HashMap},
    ops::{Add, Index},
};

use crate::solution::Solution;

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i64, i64);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
enum MapTile {
    Wall,
    Space,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn as_point(&self) -> Point {
        match self {
            Direction::North => Point(0, -1),
            Direction::South => Point(0, 1),
            Direction::West => Point(-1, 0),
            Direction::East => Point(1, 0),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}

fn parse_input(puzzle_input: String) -> (HashMap<Point, MapTile>, Point) {
    let mut hm = HashMap::new();
    let mut pos = None;
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            hm.insert(
                Point(x as i64, y as i64),
                match cell {
                    '#' => MapTile::Wall,
                    '.' => MapTile::Space,
                    'S' => {
                        pos = Some(Point(x as i64, y as i64));
                        MapTile::Space
                    }
                    'E' => MapTile::Exit,
                    x => panic!("Unknown map element {x}"),
                },
            );
        }
    }

    (hm, pos.expect("Map should have starting point"))
}

#[derive(Debug, Clone)]
struct SearchState<'map> {
    pos: Point,
    dir: Direction,
    acc_cost: u64,
    exit: Point,
    map: &'map HashMap<Point, MapTile>,
}

impl<'map> SearchState<'map> {
    fn approximate_dist(&self) -> u64 {
        self.acc_cost
            + (self.pos.0 - self.exit.0).abs() as u64
            + (self.pos.1 - self.exit.1).abs() as u64
    }

    fn neighbors(&self) -> Vec<SearchState> {
        let mut neighbors = Vec::new();

        // Can move forward
        let mut moved = self.clone();
        moved.pos = self.pos + self.dir.as_point();
        moved.acc_cost += 1;
        neighbors.push(moved);

        // Can turn left/right
        let current_dir_idx = DIRS.iter().position(|d| *d == self.dir).unwrap();
        let mut left = self.clone();
        left.dir = DIRS[(current_dir_idx - 1).rem_euclid(DIRS.len())];
        left.acc_cost += 1000;
        neighbors.push(left);

        let mut right = self.clone();
        right.dir = DIRS[(current_dir_idx + 1).rem_euclid(DIRS.len())];
        right.acc_cost += 1000;
        neighbors.push(right);

        neighbors
    }
}

impl<'map> PartialEq for SearchState<'map> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir && self.acc_cost == other.acc_cost
    }
}
impl<'map> Eq for SearchState<'map> {}

impl<'map> PartialOrd for SearchState<'map> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.approximate_dist()
                .cmp(&other.approximate_dist())
                .reverse(),
        )
    }
}

impl<'map> Ord for SearchState<'map> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct ReindeerSolution;

impl Solution for ReindeerSolution {
    fn part1(puzzle_input: String) -> String {
        let (map, start) = parse_input(puzzle_input);
        let exit = map
            .iter()
            .filter_map(|(p, t)| match t {
                MapTile::Exit => Some(p),
                _ => None,
            })
            .next()
            .unwrap();

        // Searching through (state, cost) space
        let mut frontier = BinaryHeap::new();
        frontier.push(SearchState {
            pos: start,
            dir: Direction::East,
            acc_cost: 0,
            exit: *exit,
            map: &map,
        });
        while let Some(current) = frontier.pop() {
            match map.get(&current.pos).unwrap() {
                MapTile::Exit => return current.acc_cost.to_string(),
                MapTile::Space => {
                    for neighbor in current.neighbors() {
                        frontier.push(neighbor);
                    }
                }
                _ => continue,
            }
        }

        "".to_string()
    }
    fn part2(puzzle_input: String) -> String {
        "".to_string()
    }
}
