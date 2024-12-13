use crate::solution::Solution;

#[derive(Debug, Clone)]
struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl ClawMachine {
    fn new(input: &str) -> ClawMachine {
        let lines: Vec<_> = input.trim().lines().collect();
        let (ba1, ba2) = lines[0].split_once(',').unwrap();
        let (bb1, bb2) = lines[1].split_once(',').unwrap();
        let (p1, p2) = lines[2].split_once(',').unwrap();

        ClawMachine {
            ax: ba1[12..].parse().unwrap(),
            ay: ba2[3..].parse().unwrap(),
            bx: bb1[12..].parse().unwrap(),
            by: bb2[3..].parse().unwrap(),
            px: p1[9..].parse().unwrap(),
            py: p2[3..].parse().unwrap(),
        }
    }

    fn scaled(&self) -> ClawMachine {
        let mut m = self.clone();
        m.px += 10000000000000;
        m.py += 10000000000000;
        m
    }

    fn solve(&self) -> Option<(i64, i64)> {
        // Return early if the determinent DNE
        let d_inv = self.ax * self.by - self.ay * self.bx;
        if d_inv == 0 {
            return None;
        }

        let a = ((self.px * self.by - self.py * self.bx) as f64) / (d_inv as f64);
        let b = ((self.py * self.ax - self.px * self.ay) as f64) / (d_inv as f64);

        if a.fract() > 1e-10 || b.fract() > 1e-10 {
            return None;
        }

        Some((a as i64, b as i64))
    }
}

pub struct ClawContraption;

impl Solution for ClawContraption {
    fn part1(puzzle_input: String) -> String {
        let machines: Vec<_> = puzzle_input.split("\n\n").map(ClawMachine::new).collect();
        machines
            .iter()
            .filter_map(ClawMachine::solve)
            .map(|(a, b)| a * 3 + b)
            .sum::<i64>()
            .to_string()
    }
    fn part2(puzzle_input: String) -> String {
        let machines: Vec<_> = puzzle_input.split("\n\n").map(ClawMachine::new).collect();
        machines
            .iter()
            .map(ClawMachine::scaled)
            .filter_map(|m| m.solve())
            .map(|(a, b)| a * 3 + b)
            .sum::<i64>()
            .to_string()
    }
}
