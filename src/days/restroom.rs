use std::{
    collections::HashSet,
    ops::{Add, Mul},
};

use itertools::Itertools;

use crate::solution::Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn simulate(&self, steps: i64, x_max: i64, y_max: i64) -> Robot {
        let new_pos = self.pos + (self.vel * steps);
        let scaled = Point(new_pos.0.rem_euclid(x_max), new_pos.1.rem_euclid(y_max));

        let mut new_bot = self.clone();
        new_bot.pos = scaled;

        new_bot
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i64, i64);

impl Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct RestroomSolution;

fn parse_input(puzzle_input: String) -> Vec<Robot> {
    puzzle_input
        .lines()
        .map(|l| {
            let (pstring, vstring) = l.split_once(' ').unwrap();
            let (px, py) = pstring[2..].split_once(',').unwrap();
            let (vx, vy) = vstring[2..].split_once(',').unwrap();

            Robot {
                pos: Point(px.parse().unwrap(), py.parse().unwrap()),
                vel: Point(vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect()
}

fn print_robots(robots: &[Robot], x_max: i64, y_max: i64) {
    let positions: HashSet<_> = robots.iter().map(|r| r.pos).collect();
    for y in 0..y_max {
        for x in 0..x_max {
            if positions.contains(&Point(x as i64, y as i64)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}

fn quadrant(pos: Point, x_max: i64, y_max: i64) -> Option<i64> {
    match (pos.0, pos.1) {
        (x, y) if x >= 0 && x < x_max / 2 && y >= 0 && y < y_max / 2 => Some(1),
        (x, y) if x > x_max / 2 && x < x_max && y >= 0 && y < y_max / 2 => Some(2),
        (x, y) if x >= 0 && x < x_max / 2 && y > y_max / 2 && y < y_max => Some(3),
        (x, y) if x > x_max / 2 && x < x_max && y > y_max / 2 && y < y_max => Some(4),
        _ => None,
    }
}

impl Solution for RestroomSolution {
    fn part1(puzzle_input: String) -> String {
        let robots = parse_input(puzzle_input);
        let (x_max, y_max) = if robots.len() > 12 {
            (101, 103)
        } else {
            (11, 7)
        };

        robots
            .iter()
            .map(|r| r.simulate(100, x_max, y_max))
            .filter_map(|r| quadrant(r.pos, x_max, y_max))
            .counts_by(|n| n)
            .values()
            .product::<usize>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let robots = parse_input(puzzle_input);
        let (x_max, y_max) = if robots.len() > 12 {
            (101, 103)
        } else {
            return "".to_string();
        };
        let max_steps = x_max * y_max;

        let step = (1..max_steps)
            .min_by_key(|steps| {
                let robs: Vec<_> = robots
                    .iter()
                    .map(|r| r.simulate(*steps, x_max, y_max))
                    .collect();

                robs.iter()
                    .filter_map(|r| quadrant(r.pos, x_max, y_max))
                    .counts_by(|n| n)
                    .values()
                    .product::<usize>()
            })
            .unwrap();

        let robs: Vec<_> = robots
            .iter()
            .map(|r| r.simulate(step, x_max, y_max))
            .collect();

        print_robots(&robs, x_max, y_max);

        step.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_parsing() {
        assert_eq!(
            parse_input("p=4,72 v=24,-91".to_string()),
            vec![Robot {
                pos: Point(4, 72),
                vel: Point(24, -91),
            }]
        );
    }

    #[test]
    fn robots_move_correctly() {
        let r = Robot {
            pos: Point(2, 4),
            vel: Point(2, -3),
        };

        assert_eq!(r.simulate(1, 11, 7).pos, Point(4, 1));
        assert_eq!(r.simulate(2, 11, 7).pos, Point(6, 5));
        assert_eq!(r.simulate(3, 11, 7).pos, Point(8, 2));
        assert_eq!(r.simulate(5, 11, 7).pos, Point(1, 3));
    }

    #[test]
    fn quadrants_are_correct() {
        assert_eq!(quadrant(Point(0, 0), 3, 3), Some(1));
        assert_eq!(quadrant(Point(1, 0), 3, 3), None);
        assert_eq!(quadrant(Point(2, 0), 3, 3), Some(2));
        assert_eq!(quadrant(Point(0, 1), 3, 3), None);
        assert_eq!(quadrant(Point(1, 1), 3, 3), None);
        assert_eq!(quadrant(Point(2, 1), 3, 3), None);
        assert_eq!(quadrant(Point(0, 2), 3, 3), Some(3));
        assert_eq!(quadrant(Point(1, 2), 3, 3), None);
        assert_eq!(quadrant(Point(2, 2), 3, 3), Some(4));
    }
}
