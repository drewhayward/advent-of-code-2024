use std::{
    collections::{HashSet, VecDeque}, fmt::format, ops::Add
};

use crate::solution::Solution;

const DIRS: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn neighbors(p: Point, size: i32) -> Vec<Point> {
    let mut neighbors = Vec::new();

    for dir in DIRS {
        let neighbor = p + dir;

        if neighbor.0 >= 0 && neighbor.0 <= size && neighbor.1 >= 0 && neighbor.1 <= size {
            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn parse_input(puzzle_input: String) -> Vec<Point> {
    puzzle_input
        .lines()
        .map(|l| {
            let nums: Vec<_> = l
                .split(',')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect();

            Point(nums[0], nums[1])
        })
        .collect()
}

fn shortest_path(barriers: &[Point], size: i32) -> Option<i32> {
    let dropped_barriers: HashSet<Point> = barriers.iter().cloned().collect();

    let goal = Point(size, size);

    let mut frontier = VecDeque::new();
    frontier.push_back((0, Point(0, 0)));
    let mut visited = HashSet::new();
    while let Some((t, pos)) = frontier.pop_front() {
        if pos == goal {
            return Some(t);
        }

        if visited.contains(&pos) {
            continue;
        } else {
            visited.insert(pos);
        }

        for neighbor in neighbors(pos, size) {
            // Skip this spot if the barrier has fallen
            if dropped_barriers.contains(&neighbor) {
                continue;
            }

            frontier.push_back((t + 1, neighbor));
        }
    }

    None
}

pub struct RamRunSolution;

impl Solution for RamRunSolution {
    fn part1(puzzle_input: String) -> String {
        let barriers = parse_input(puzzle_input);
        let (size, num_barriers) = if barriers.len() <= 25 {
            (6, 12)
        } else {
            (70, 1024)
        };

        let solution = shortest_path(&barriers[..num_barriers], size);

        solution.unwrap().to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let barriers = parse_input(puzzle_input);
        let size = if barriers.len() <= 25 { 6 } else { 70 };

        let mut start = 0; // Inclusive
        let mut end = barriers.len(); // Exclusive
        while start < end {
            let mid = (start + end) / 2;
            // Mid is included in the test
            let solvable = shortest_path(&barriers[..mid + 1], size);

            if solvable.is_some() {
                start = mid + 1;
            } else {
                end = mid
            }
        }

        let breakpoint = barriers.get(start).unwrap();
        format!("{},{}", breakpoint.0, breakpoint.1)
    }
}
