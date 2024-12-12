use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge {
    start: Point,
    end: Point,
}

impl Edge {
    fn new(point: Point, dir: Direction) -> Edge {
        match dir {
            Direction::Up => Edge {
                start: point,
                end: point + Direction::Right.as_point(),
            },
            Direction::Left => Edge {
                start: point,
                end: point + Direction::Down.as_point(),
            },
            Direction::Right => Edge {
                start: point + Direction::Right.as_point(),
                end: point + Direction::Right.as_point() + Direction::Down.as_point(),
            },
            Direction::Down => Edge {
                start: point + Direction::Down.as_point(),
                end: point + Direction::Down.as_point() + Direction::Right.as_point(),
            },
        }
    }

    fn merge(a: Edge, b: Edge) -> Edge {
        if b.end == a.start {
            Edge {
                start: b.start,
                end: a.end,
            }
        } else if a.end == b.start {
            Edge {
                start: a.start,
                end: b.end,
            }
        } else {
            panic!("Calling merge for edges which aren't compatible")
        }
    }

    fn is_horizontal(&self) -> bool {
        assert!(self.start != self.end);
        return self.start.1 == self.end.1;
    }

    // Returns a vector of edges that could extend this edge, respected all_edges
    fn possible_extensions(&self, all_edges: &HashSet<Edge>) -> Vec<Edge> {
        let mut extensions = Vec::new();
        match self.is_horizontal() {
            true => {
                let candidate1 = Edge {
                    start: self.start + Direction::Left.as_point(),
                    end: self.start,
                };
                let breaking_edge1 = Edge {
                    start: self.start + Direction::Up.as_point(),
                    end: self.start,
                };
                let candidate2 = Edge {
                    start: self.end,
                    end: self.end + Direction::Right.as_point(),
                };
                let breaking_edge2 = Edge {
                    start: self.end + Direction::Up.as_point(),
                    end: self.end,
                };

                if !all_edges.contains(&breaking_edge1) {
                    extensions.push(candidate1);
                }
                if !all_edges.contains(&breaking_edge2) {
                    extensions.push(candidate2);
                }
            }
            false => {
                let candidate1 = Edge {
                    start: self.start + Direction::Up.as_point(),
                    end: self.start,
                };
                let breaking_edge1 = Edge {
                    start: self.start,
                    end: self.start + Direction::Right.as_point(),
                };
                let candidate2 = Edge {
                    start: self.end,
                    end: self.end + Direction::Down.as_point(),
                };
                let breaking_edge2 = Edge {
                    start: self.end,
                    end: self.end + Direction::Right.as_point(),
                };

                if !all_edges.contains(&breaking_edge1) {
                    extensions.push(candidate1);
                }
                if !all_edges.contains(&breaking_edge2) {
                    extensions.push(candidate2);
                }
            }
        }

        extensions
    }
}

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

    fn all_dirs() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

fn parse_input(puzzle_input: String) -> HashMap<Point, char> {
    let mut hm = HashMap::new();
    for (y, row) in puzzle_input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            hm.insert(Point(x as i32, y as i32), cell);
        }
    }

    hm
}

fn get_area_and_perimeter_and_edges(
    map: &HashMap<Point, char>,
    start: Point,
    to_visit: &mut HashSet<Point>,
) -> (u64, u64, HashSet<Edge>) {
    let region = map.get(&start).unwrap();
    let mut area = HashSet::new();
    let mut frontier = vec![start];
    let mut perimeter = 0;
    let mut edges: HashSet<Edge> = HashSet::new();

    while let Some(point) = frontier.pop() {
        if area.contains(&point) {
            continue;
        }

        // We must have a new point
        area.insert(point);
        to_visit.remove(&point);

        for dir in Direction::all_dirs() {
            let neighbor = point + dir.as_point();

            match map.get(&neighbor) {
                Some(r) if r != region => {
                    perimeter += 1;
                    edges.insert(Edge::new(point, dir));
                    continue;
                }
                None => {
                    perimeter += 1;
                    edges.insert(Edge::new(point, dir));
                    continue;
                }
                _ => {}
            }

            frontier.push(neighbor);
        }
    }

    (area.len() as u64, perimeter, edges)
}

fn count_sides(all_edges: &HashSet<Edge>) -> u64 {
    let mut edges_to_merge = all_edges.clone();
    let mut merged_edges = HashSet::new();

    // While we have unused edges
    while let Some(edge) = edges_to_merge.iter().cloned().next() {
        // Take a primitive edge
        let mut current_edge = edges_to_merge.take(&edge).unwrap();

        // Loop until you can't extend it no mo'
        loop {
            let mut did_extend = false;
            for extension in current_edge.possible_extensions(all_edges) {
                if !edges_to_merge.contains(&extension) {
                    continue;
                }

                // Claim the primitive edge and merge
                edges_to_merge.remove(&extension);
                current_edge = Edge::merge(current_edge, extension);
                did_extend = true
            }

            if !did_extend {
                break;
            }
        }

        merged_edges.insert(current_edge);
    }

    merged_edges.len() as u64
}

pub struct GardenGroupSolution;

impl Solution for GardenGroupSolution {
    fn part1(puzzle_input: String) -> String {
        let map = parse_input(puzzle_input);

        let mut to_visit: HashSet<_> = map.keys().cloned().collect();
        let mut result = 0;
        while let Some(point) = to_visit.iter().next().cloned() {
            let (area, perimeter, _) = get_area_and_perimeter_and_edges(&map, point, &mut to_visit);
            result += area * perimeter
        }

        result.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let map = parse_input(puzzle_input);

        let mut to_visit: HashSet<_> = map.keys().cloned().collect();
        let mut result = 0;
        while let Some(point) = to_visit.iter().next().cloned() {
            let (area, _, edges) = get_area_and_perimeter_and_edges(&map, point, &mut to_visit);
            let num_sides = count_sides(&edges);

            result += area * num_sides
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::garden::GardenGroupSolution, solution::Solution};

    #[test]
    fn case1() {
        let example = "AAAA
BBCD
BBCC
EEEC"
            .to_string();

        assert_eq!(GardenGroupSolution::part2(example), "80")
    }

    #[test]
    fn case2() {
        let example = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            .to_string();

        assert_eq!(GardenGroupSolution::part2(example), "436")
    }

    #[test]
    fn case3() {
        let example = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            .to_string();

        assert_eq!(GardenGroupSolution::part2(example), "236")
    }

    #[test]
    fn case4() {
        let example = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            .to_string();

        assert_eq!(GardenGroupSolution::part2(example), "368")
    }
}
