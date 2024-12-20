use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

use itertools::Itertools;

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MapTile {
    Wall,
    Space,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn as_point(&self) -> Point {
        match self {
            Direction::North => Point(0, -1),
            Direction::South => Point(0, 1),
            Direction::East => Point(-1, 0),
            Direction::West => Point(1, 0),
        }
    }
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

/// A node in the search space
/// `node.0`: the position in the map
/// `node.1`: the number of wall tiles we are allowed to move through
/// `node.2`: the number of steps taken to get to this point
type Node = (Point, u32, u32);

fn parse_input(puzzle_input: String) -> (HashMap<Point, MapTile>, Point, Point) {
    let mut hm = HashMap::new();
    let mut start = None;
    let mut end = None;
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            let cell_pos = Point(x as i32, y as i32);
            hm.insert(
                cell_pos,
                match cell {
                    '#' => MapTile::Wall,
                    'S' => {
                        start = Some(cell_pos);
                        MapTile::Space
                    }
                    '.' => MapTile::Space,
                    'E' => {
                        end = Some(cell_pos);
                        MapTile::Space
                    }
                    x => panic!("Unknown map element {x}"),
                },
            );
        }
    }

    (
        hm,
        start.expect("Map should have starting point"),
        end.expect("map should have an ending point"),
    )
}

fn neighbors(node: &Point, map: &HashMap<Point, MapTile>) -> Vec<Point> {
    let mut neighbors = Vec::with_capacity(4);

    for dir in DIRS {
        let neighbor = *node + dir.as_point();
        let tile = map.get(&neighbor).unwrap();
        // cheats timer should run regardless of if we are going through a wall or not
        match tile {
            MapTile::Space => neighbors.push(neighbor),
            _ => {}
        }
    }

    neighbors
}

fn find_min_dists(map: &HashMap<Point, MapTile>, start: Point) -> HashMap<Point, u64> {
    let mut min_dists: HashMap<Point, u64> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((node, dist)) = queue.pop_front() {
        if min_dists.contains_key(&node) {
            continue;
        } else {
            min_dists.insert(node, dist);
        }

        for neighbor in neighbors(&node, &map) {
            queue.push_back((neighbor, dist + 1));
        }
    }

    min_dists
}

type Cheat = (Point, Point);

fn option_add(lhs: Option<u64>, rhs: Option<u64>) -> Option<u64> {
    match (lhs, rhs) {
        (Some(l), Some(r)) => Some(l + r),
        _ => None,
    }
}

pub struct RaceCondition;

impl Solution for RaceCondition {
    fn part1(puzzle_input: String) -> String {
        let (map, start, end) = parse_input(puzzle_input);
        let min_start_dists = find_min_dists(&map, start);
        let min_end_dists = find_min_dists(&map, end);

        let dist_to_beat = min_start_dists.get(&end).unwrap();

        let mut cheat_legend: HashMap<Cheat, u64> = HashMap::new();

        let walls = map.iter().filter_map(|(p, t)| match t {
            MapTile::Wall => Some(p),
            _ => None,
        });

        // Look for cheats
        for wall_pos in walls {
            // Lookup the shortest way to get to this wall
            let start_dist = match DIRS
                .iter()
                .filter_map(|start_dir| {
                    let cheat_start = *wall_pos + start_dir.as_point();
                    match min_start_dists.get(&cheat_start) {
                        Some(dist) => Some(dist + 1),
                        None => None,
                    }
                })
                .min()
            {
                Some(d) => d,
                None => continue,
            };

            // Check for adjacent places we could shortcut to
            for end_dir in DIRS {
                let cheat_end = *wall_pos + end_dir.as_point();
                let end_dist = match min_end_dists.get(&cheat_end) {
                    Some(dist) => dist,
                    None => continue,
                };

                let cheat_dist = 1 + start_dist + end_dist;

                if *dist_to_beat > cheat_dist {
                    cheat_legend.insert((*wall_pos, cheat_end), *dist_to_beat - cheat_dist);
                }
            }
        }

        cheat_legend
            .values()
            .filter(|d| **d >= 100)
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (map, start, end) = parse_input(puzzle_input);
        let min_start_dists = find_min_dists(&map, start);
        let min_end_dists = find_min_dists(&map, end);

        let dist_to_beat = min_start_dists.get(&end).unwrap();

        let mut cheat_legend: HashMap<Cheat, u64> = HashMap::new();

        let spaces = map.iter().filter_map(|(p, t)| match t {
            MapTile::Space => Some(p),
            _ => None,
        });

        // Look for cheats
        for space_pos in spaces {
            // Lookup the shortest way to get to this space
            let start_dist = match min_start_dists.get(&space_pos) {
                Some(d) => d,
                None => continue,
            };

            // BFS for the places we could shortcut to
            let mut visited = HashSet::new();
            let mut frontier = VecDeque::new();
            frontier.push_back((*space_pos, 0));
            //frontier.extend(DIRS.iter().filter_map(|dir| {
            //    let neighbor = *space_pos + dir.as_point();
            //    if let Some(MapTile::Wall) = map.get(&neighbor){
            //        Some((neighbor, 1))
            //    } else {
            //        None
            //    }
            //}));
            while let Some((position, dist)) = frontier.pop_front() {
                if visited.contains(&position) {
                    continue;
                } else {
                    visited.insert(position);
                }

                // Check for a cheat end
                if let Some(end_dist) = min_end_dists.get(&position) {
                    let cheat_dist = dist + start_dist + end_dist;
                    if *dist_to_beat > cheat_dist {
                        cheat_legend.insert((*space_pos, position), *dist_to_beat - cheat_dist);
                    }
                }
                
                if dist > 19 {
                    continue;
                }

                // Neighborhood exploration
                for neighbor_dir in DIRS {
                    let neighbor = position + neighbor_dir.as_point();
                    if let Some(_) = map.get(&neighbor) {
                        frontier.push_back((neighbor, dist + 1));
                    }
                }
            }
        }


        cheat_legend
            .values()
            .filter(|d| **d >= 100)
            .count()
            .to_string()
    }
}
