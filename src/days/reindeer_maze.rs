use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
    u64,
};

use crate::{solution::Solution, utils::wait_for_input};

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum MapTile {
    Wall,
    Hall,
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

    fn left(&self) -> Direction {
        let current_dir_idx = DIRS.iter().position(|d| *d == *self).unwrap() as isize;
        DIRS[(current_dir_idx - 1).rem_euclid(DIRS.len() as isize) as usize]
    }

    fn right(&self) -> Direction {
        let current_dir_idx = DIRS.iter().position(|d| *d == *self).unwrap() as isize;
        DIRS[(current_dir_idx + 1).rem_euclid(DIRS.len() as isize) as usize]
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
                    '.' => MapTile::Hall,
                    'S' => {
                        pos = Some(Point(x as i64, y as i64));
                        MapTile::Hall
                    }
                    'E' => MapTile::Exit,
                    x => panic!("Unknown map element {x}"),
                },
            );
        }
    }

    (hm, pos.expect("Map should have starting point"))
}

#[derive(Debug, Clone, Hash)]
struct SearchState {
    pos: Point,
    dir: Direction,
    acc_cost: u64,
    exit: Point,
}

impl SearchState {
    fn approximate_dist(&self) -> u64 {
        // Turns are the most expensive action, so we should try to accurately estimate them
        let x_diff = (self.pos.0 - self.exit.0).abs();
        let y_diff = (self.pos.1 - self.exit.1).abs();

        let turns = if (x_diff != 0) || (y_diff != 0) {
            1000
        } else {
            0
        };

        self.acc_cost + x_diff as u64 + y_diff as u64 + (turns * 1000)
    }
}

fn neighbors(node: &Node) -> Vec<(Node, u64)> {
    let (pos, dir) = node;
    let mut neighbors = Vec::new();

    // March forward and create a new state for each new branch encounter
    neighbors.push(((*pos + dir.as_point(), *dir), 1));

    // Can turn left/right
    neighbors.push(((*pos, dir.left()), 1000));
    neighbors.push(((*pos, dir.right()), 1000));

    neighbors
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir && self.acc_cost == other.acc_cost
    }
}
impl Eq for SearchState {}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.approximate_dist()
                .cmp(&other.approximate_dist())
                .reverse(),
        )
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type Node = (Point, Direction);

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

        let mut unvisited: HashSet<Node> = map
            .iter()
            .filter_map(|(p, t)| (*t != MapTile::Wall).then_some(p))
            .flat_map(|p| {
                vec![
                    (*p, Direction::North),
                    (*p, Direction::South),
                    (*p, Direction::East),
                    (*p, Direction::West),
                ]
            })
            .collect();

        let mut min_dist: HashMap<Node, u64> =
            unvisited.iter().cloned().map(|n| (n, u64::MAX)).collect();
        min_dist.insert((start, Direction::East), 0);

        while !unvisited.is_empty() {
            println!("{}", unvisited.len());
            let current = unvisited.iter().min_by_key(|n| min_dist.get(&n)).unwrap().clone();
            let current_cost = min_dist.get(&current).unwrap().clone();

            for (neighbor, transition_cost) in neighbors(&current) {
                if unvisited.contains(&neighbor) {
                    let neighbor_cost = min_dist.get(&neighbor).unwrap().clone();

                    min_dist.insert(neighbor, neighbor_cost.min(current_cost + transition_cost));
                }
            }

            unvisited.remove(&current);
        }

        vec![
            (*exit, Direction::North),
            (*exit, Direction::South),
            (*exit, Direction::East),
            (*exit, Direction::West),
        ]
        .iter()
        .map(|n| min_dist.get(n).unwrap())
        .min()
        .unwrap()
        .to_string()
    }
    fn part2(puzzle_input: String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn name() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();

        assert_eq!(ReindeerSolution::part1(input), "7036".to_string());
    }
}
