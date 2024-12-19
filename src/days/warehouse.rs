use core::panic;
use std::{collections::HashMap, ops::Add};

use crate::{solution::Solution, utils::wait_for_input};

const DIRS: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point(i64, i64);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
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
}

#[derive(Clone, PartialEq, Eq)]
enum MapTile {
    Wall,
    Box,
    Space,
}

fn parse_input(puzzle_input: String) -> (HashMap<Point, MapTile>, Vec<Direction>, Point) {
    let (map_string, move_string) = puzzle_input.split_once("\n\n").unwrap();
    let mut robot = None;

    let mut map = HashMap::new();
    for (y, row) in map_string.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let pos = Point(x as i64, y as i64);
            let tile = match c {
                '#' => MapTile::Wall,
                'O' => MapTile::Box,
                '.' => MapTile::Space,
                '@' => {
                    robot = Some(pos);
                    MapTile::Space
                }
                _ => panic!("Unexpected map character {c}"),
            };
            map.insert(pos, tile);
        }
    }

    let moves: Vec<_> = move_string
        .trim()
        .chars()
        .filter_map(|move_| match move_ {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '\n' => None,
            _ => panic!("bad direction '{move_}'"),
        })
        .collect();

    (map, moves, robot.unwrap())
}

fn print_map(map: &HashMap<Point, MapTile>, robot: &Point) {
    let x_min = map.keys().map(|p| p.0).max().unwrap();
    let y_min = map.keys().map(|p| p.1).max().unwrap();

    for y in 0..y_min + 1 {
        for x in 0..x_min + 1 {
            let p = Point(x,y);
            if p == *robot {
                print!("@");
                continue;
            }

            let c = match map.get(&p).unwrap() {
                MapTile::Wall => "#",
                MapTile::Space => ".",
                MapTile::Box => "O",
            };
            print!("{c}");
        }
        print!("\n");
    }
}

pub struct WarehouseSolution;

impl Solution for WarehouseSolution {
    fn part1(puzzle_input: String) -> String {
        let (mut map, moves, robot) = parse_input(puzzle_input);

        let mut current_position = robot.clone();
        for move_ in moves {
            //print_map(&map, &current_position);
            //wait_for_input();

            let target_position = current_position + move_.as_point();
            // Scan in the move direction over any boxes until a space is found
            let mut empty_spot = target_position.clone();
            while let Some(MapTile::Box) = map.get(&empty_spot) {
                empty_spot = empty_spot + move_.as_point();
            }

            // If the non-box is not a space, we can't move!
            if map.get(&empty_spot).cloned() != Some(MapTile::Space) {
                continue;
            }

            // We need to swap the boxes
            if empty_spot != target_position {
                map.insert(target_position, MapTile::Space);
                map.insert(empty_spot, MapTile::Box);
            }
            current_position = target_position;
        }

        //print_map(&map, &current_position);

        map.iter()
            .filter_map(|(point, tile)| match tile {
                MapTile::Box => Some(point.1 * 100 + point.0),
                _ => None,
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        String::new()
    }
}
