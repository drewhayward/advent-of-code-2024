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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Wall,
    Box,
    Space,
    LBox,
    RBox,
}

/// Find the 'root' coord for a 2-wide box
fn root(map: &HashMap<Point, MapTile>, p: Point) -> Point {
    match map.get(&p).unwrap() {
        MapTile::LBox => p,
        MapTile::RBox => p + Direction::Left.as_point(),
        x => panic!("calling root for {:?}", x),
    }
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

fn expand_map(map: HashMap<Point, MapTile>) -> HashMap<Point, MapTile> {
    map.iter()
        .flat_map(|(p, tile)| {
            let (ltile, rtile) = match tile.clone() {
                MapTile::Box => (MapTile::LBox, MapTile::RBox),
                t => (t, t),
            };

            vec![
                (Point(p.0 * 2, p.1), ltile),
                (Point(p.0 * 2 + 1, p.1), rtile),
            ]
        })
        .collect()
}

fn print_map(map: &HashMap<Point, MapTile>, robot: &Point) {
    let x_min = map.keys().map(|p| p.0).max().unwrap();
    let y_min = map.keys().map(|p| p.1).max().unwrap();

    for y in 0..y_min + 1 {
        for x in 0..x_min + 1 {
            let p = Point(x, y);
            if p == *robot {
                print!("@");
                continue;
            }

            let c = match map.get(&p).unwrap() {
                MapTile::Wall => "#",
                MapTile::Space => ".",
                MapTile::Box => "O",
                MapTile::LBox => "[",
                MapTile::RBox => "]",
            };
            print!("{c}");
        }
        print!("\n");
    }
}

fn can_move_box(candidate: Point, dir: Direction, map: &HashMap<Point, MapTile>) -> bool {
    match map.get(&candidate) {
        Some(MapTile::Space) => return true,
        Some(MapTile::Box) => return can_move_box(candidate + dir.as_point(), dir, map),
        Some(MapTile::LBox) | Some(MapTile::RBox) => match dir {
            // L/R behaves normally
            Direction::Left | Direction::Right => {
                return can_move_box(candidate + dir.as_point(), dir, map)
            }
            // Also check the siblings movability
            _ => {
                let root_pos = root(map, candidate);
                let sibling_pos = root_pos + Direction::Right.as_point();

                can_move_box(root_pos + dir.as_point(), dir, map)
                    && can_move_box(sibling_pos + dir.as_point(), dir, map)
            }
        },
        _ => return false,
    }
}

/// Called on a root movable map tile which recurses to move all the necessary knock-on tiles
fn move_box(candidate: Point, dir: Direction, map: &mut HashMap<Point, MapTile>) {
    let tile = map.get(&candidate).unwrap();
    match tile {
        MapTile::Wall => panic!("Tried to move into a wall"),
        MapTile::Box => {
            // First make room
            move_box(candidate + dir.as_point(), dir, map);

            // Then move this thing
            let tile = map.get(&candidate).unwrap().clone();
            map.insert(candidate, MapTile::Space);
            map.insert(candidate + dir.as_point(), tile);
        }
        MapTile::RBox | MapTile::LBox => match dir {
            // Move dependencies
            Direction::Left | Direction::Right => {
                move_box(candidate + dir.as_point(), dir, map);
                let tile = map.get(&candidate).unwrap().clone();
                map.insert(candidate, MapTile::Space);
                map.insert(candidate + dir.as_point(), tile);
            }
            // Need to handle the cases for touching boxes
            _ => {
                let root_pos = root(map, candidate);
                let sibling_pos = root_pos + Direction::Right.as_point();

                // No matter what, move the left one
                move_box(root_pos + dir.as_point(), dir, map);

                // If the right one is a diff box, also call move on it
                if let Some(MapTile::LBox) = map.get(&(sibling_pos + dir.as_point())) {
                    move_box(sibling_pos + dir.as_point(), dir, map);
                }

                // Then move myself and my sibling thing
                let tile = map.get(&root_pos).unwrap().clone();
                map.insert(root_pos, MapTile::Space);
                map.insert(root_pos + dir.as_point(), tile);
                let tile = map.get(&sibling_pos).unwrap().clone();
                map.insert(sibling_pos, MapTile::Space);
                map.insert(sibling_pos + dir.as_point(), tile);
            }
        },
        _ => {}
    };
}

pub struct WarehouseSolution;

impl Solution for WarehouseSolution {
    fn part1(puzzle_input: String) -> String {
        let (mut map, moves, robot) = parse_input(puzzle_input);

        let mut current_position = robot.clone();
        for move_ in moves {
            let target_position = current_position + move_.as_point();
            if can_move_box(target_position, move_, &map) {
                move_box(target_position, move_, &mut map);
                current_position = target_position;
            }
        }

        map.iter()
            .filter_map(|(point, tile)| match tile {
                MapTile::Box => Some(point.1 * 100 + point.0),
                _ => None,
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (mut map, moves, mut robot) = parse_input(puzzle_input);
        robot.0 = robot.0 * 2;
        map = expand_map(map);

        let mut current_position = robot.clone();
        for move_ in moves {
            //print_map(&map, &current_position);
            //wait_for_input();

            let target_position = current_position + move_.as_point();
            if can_move_box(target_position, move_, &map) {
                move_box(target_position, move_, &mut map);
                current_position = target_position;
            }
        }

        print_map(&map, &current_position);

        map.iter()
            .filter_map(|(point, tile)| match tile {
                MapTile::LBox => Some(point.1 * 100 + point.0),
                _ => None,
            })
            .sum::<i64>()
            .to_string()
    }
}
