use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    vec,
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

#[derive(Debug, Clone, Copy)]
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
    fn all_dirs() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

fn parse_input(puzzle_input: String) -> HashMap<Point, u32> {
    let mut hm = HashMap::new();
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            hm.insert(Point(x as i32, y as i32), cell.to_string().parse().unwrap());
        }
    }
    hm
}

fn count_trails(map: &HashMap<Point, u32>, trailhead: &Point) -> u32 {
    let mut summits = HashSet::new();
    let mut frontier = vec![(*trailhead, 0)];
    while let Some((position, height)) = frontier.pop() {
        if height == 9 {
            summits.insert(position);
        }
        // Queue neighbors
        for dir in Direction::all_dirs().iter() {
            let neighbor = position + dir.as_point();
            match map.get(&neighbor) {
                Some(neighbor_height) if *neighbor_height == height + 1 => {
                    frontier.push((neighbor, *neighbor_height))
                }
                _ => {}
            }
        }
    }

    summits.len() as u32
}

fn rate_trailhead(map: &HashMap<Point, u32>, trailhead: &Point, current_height: u32) -> u32 {
    if current_height == 9 {
        return 1;
    }

    let mut total = 0;
    for dir in Direction::all_dirs().iter() {
        let neighbor = *trailhead + dir.as_point();
        match map.get(&neighbor) {
            Some(v) if *v == current_height + 1 => total += rate_trailhead(&map, &neighbor, *v),
            _ => {}
        }
    }

    total
}

pub struct HoofItSolution;

impl Solution for HoofItSolution {
    fn part1(puzzle_input: String) -> String {
        let trail_map = parse_input(puzzle_input);

        let trailheads = trail_map.iter().filter_map(|(p, v)| match v {
            0 => Some(p),
            _ => None,
        });

        trailheads
            .map(|t| count_trails(&trail_map, t))
            .sum::<u32>()
            .to_string()
    }
    fn part2(puzzle_input: String) -> String {
        let trail_map = parse_input(puzzle_input);

        let trailheads = trail_map.iter().filter_map(|(p, v)| match v {
            0 => Some(p),
            _ => None,
        });

        trailheads
            .map(|t| rate_trailhead(&trail_map, t, 0))
            .sum::<u32>()
            .to_string()
    }
}
