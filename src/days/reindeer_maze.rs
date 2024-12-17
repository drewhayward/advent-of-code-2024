use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
    u64,
};

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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

fn parse_input(puzzle_input: String) -> (HashMap<Point, MapTile>, Point, Point) {
    let mut hm = HashMap::new();
    let mut start = None;
    let mut exit = None;
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            hm.insert(
                Point(x as i64, y as i64),
                match cell {
                    '#' => MapTile::Wall,
                    '.' => MapTile::Hall,
                    'S' => {
                        start = Some(Point(x as i64, y as i64));
                        MapTile::Hall
                    }
                    'E' => {
                        exit = Some(Point(x as i64, y as i64));
                        MapTile::Hall
                    }
                    x => panic!("Unknown map element {x}"),
                },
            );
        }
    }

    (
        hm,
        start.expect("Map should have starting point"),
        exit.expect("Map should have an exit"),
    )
}

fn neighbors(node: &Node, map: &HashMap<Point, MapTile>) -> Vec<(Node, u64)> {
    let (pos, dir) = node;
    let mut neighbors = Vec::new();

    let new_pos = *pos + dir.as_point();
    if map.get(&new_pos) != Some(&MapTile::Wall) {
        neighbors.push(((new_pos, *dir), 1));
    }

    // Can turn left/right
    neighbors.push(((*pos, dir.left()), 1000));
    neighbors.push(((*pos, dir.right()), 1000));

    neighbors
}

fn neighbors_reversed(node: &Node, map: &HashMap<Point, MapTile>) -> Vec<(Node, u64)> {
    let (pos, dir) = node;
    let mut neighbors = Vec::new();

    let new_pos = *pos + dir.right().right().as_point();
    if map.get(&new_pos) != Some(&MapTile::Wall) {
        neighbors.push(((new_pos, *dir), 1));
    }

    // Can turn left/right
    neighbors.push(((*pos, dir.left()), 1000));
    neighbors.push(((*pos, dir.right()), 1000));

    neighbors
}

type Node = (Point, Direction);
type QueueNode = (u64, Node);

fn expand(p: Point) -> Vec<Node> {
    vec![
        (p, Direction::North),
        (p, Direction::South),
        (p, Direction::East),
        (p, Direction::West),
    ]
}

pub struct ReindeerSolution;

fn find_min_dists(map: &HashMap<Point, MapTile>, start: Point) -> HashMap<(Point, Direction), u64> {
    let nodes: Vec<Node> = map
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

    let start_node = (start, Direction::East);

    // The min dist map stores persistent answer
    let mut min_dist: HashMap<Node, u64> = nodes.iter().cloned().map(|n| (n, u64::MAX)).collect();
    min_dist.insert(start_node, 0);

    // The queue could have duplicate nodes and is just to quickly find the next node to visit
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_node)));

    while let Some(Reverse((cost, node))) = queue.pop() {
        //println!("Popping {:?} at cost {:?}", node, cost);

        for (neighbor, transition_cost) in neighbors(&node, &map) {
            let new_neighbor_cost = cost + transition_cost;
            //println!(" Looking at neighbor {:?} with a cost of {}", neighbor, new_neighbor_cost);
            let best_neighbor_cost = min_dist.get(&neighbor).unwrap().clone();

            if new_neighbor_cost < best_neighbor_cost {
                //println!("  Neighbor has better cost than previously observed, adding");
                // Update the neighbor cost
                min_dist.insert(neighbor, new_neighbor_cost);

                // Queue the neighbor to be evaluated
                queue.push(Reverse((new_neighbor_cost, neighbor)));
            }
        }
    }

    min_dist
}

impl Solution for ReindeerSolution {
    fn part1(puzzle_input: String) -> String {
        let (map, start, exit) = parse_input(puzzle_input);

        let min_dists = find_min_dists(&map, start);

        expand(exit)
            .iter()
            .map(|n| min_dists.get(n).unwrap())
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (map, start, exit) = parse_input(puzzle_input);
        let min_dists = find_min_dists(&map, start);
        let starting_nodes = expand(exit);

        let best_dist = expand(exit)
            .iter()
            .map(|n| min_dists.get(n).unwrap())
            .min()
            .unwrap();

        // Reverse-search through the nodes to find any which could be part of a best path
        let mut visited = HashSet::new();
        // Frontier contains nodes, dist which are known to be optimal in dist and location.
        let mut frontier: Vec<(Node, u64)> = starting_nodes
            .iter()
            .cloned()
            .filter_map(|n| match min_dists.get(&n) {
                Some(dist) if dist == best_dist => Some((n, *best_dist)),
                _ => None,
            })
            .collect();

        while let Some((node, dist)) = frontier.pop() {
            visited.insert(node);
            for (neighbor, cost) in neighbors_reversed(&node, &map) {
                let neighbor_dist = min_dists.get(&neighbor).unwrap();

                if dist.checked_sub(cost) == Some(*neighbor_dist) {
                    frontier.push((neighbor, *neighbor_dist));
                }
            }
        }

        let tiles: HashSet<_> = visited.iter().map(|(p, _)| p).collect();

        tiles.len().to_string()
    }
}

#[cfg(test)]
mod test {
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
