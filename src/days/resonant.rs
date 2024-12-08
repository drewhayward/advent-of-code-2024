use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
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

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone)]
enum MapCell {
    Empty,
    Antenna(char),
}

fn parse_input(puzzle_input: String) -> HashMap<Point, MapCell> {
    let mut hm = HashMap::new();
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            hm.insert(
                Point(x as i32, y as i32),
                match cell {
                    '.' => MapCell::Empty,
                    x if x.is_alphanumeric() => MapCell::Antenna(x),
                    x => panic!("Unknown map element {x}"),
                },
            );
        }
    }

    hm
}

fn build_freq_locations(antenna_map: &HashMap<Point, MapCell>) -> HashMap<&char, Vec<Point>>{
    let mut freq_locations = HashMap::new();
    for (pos, cell) in antenna_map {
        if let MapCell::Antenna(a) = cell {
            freq_locations
                .entry(a)
                .or_insert(Vec::new())
                .push(pos.clone());
        }
    }
    freq_locations
}

pub struct Resonant;

impl Solution for Resonant {
    fn part1(puzzle_input: String) -> String {
        let antenna_map = parse_input(puzzle_input);
        let freq_locations = build_freq_locations(&antenna_map);

        // find each pair of same-freq antenna
        let mut antinodes = HashSet::new();
        for (_freq, locations) in freq_locations {
            // For each pair, calculate the two points
            for pair in locations.iter().combinations(2) {
                let a = pair[0];
                let b = pair[1];
                let diff = *a - *b;
                antinodes.insert(*a + diff);
                antinodes.insert(*a - diff - diff);
            }
        }

        antinodes
            .iter()
            .filter(|a| match &antenna_map.get(a) {
                Some(MapCell::Antenna(_)) => true,
                Some(_) => true,
                None => false,
            })
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let antenna_map = parse_input(puzzle_input);
        let freq_locations = build_freq_locations(&antenna_map);

        // find each pair of same-freq antenna
        let mut antinodes = HashSet::new();
        for (_freq, locations) in freq_locations {
            // For each pair, calculate the two points
            for pair in locations.iter().combinations(2) {
                let a = pair[0];
                let b = pair[1];
                let diff = *a - *b;
                // Go until we leave the map in either direction
                let mut current = *a;
                while let Some(_) = &antenna_map.get(&current) {
                    antinodes.insert(current);
                    current = current + diff;
                }

                current = *a;
                while let Some(_) = &antenna_map.get(&current) {
                    antinodes.insert(current);
                    current = current - diff;
                }
            }
        }

        antinodes
            .iter()
            .filter(|a| match &antenna_map.get(a) {
                Some(MapCell::Antenna(_)) => true,
                Some(_) => true,
                None => false,
            })
            .count()
            .to_string()
    }
}
