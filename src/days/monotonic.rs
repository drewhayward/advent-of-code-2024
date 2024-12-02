use crate::solution::Solution;

pub struct MonotonicReport;

impl MonotonicReport {
    fn is_monotonic(nums: &[i32], tolerance: i32, faults: usize) -> bool {
        let mut increasing = 0;
        let mut decreasing = 0;

        let mut current = nums[0];
        for num in nums.iter().skip(1) {
            match *num - current {
                x if x > 0 && x.abs() < tolerance => {
                    increasing += 1;
                }
                x if x < 0 && x.abs() < tolerance => {
                    decreasing += 1;
                }
                _ => {}
            }
            current = *num;
        }

        println!("{increasing}, {decreasing}, {}", nums.len());

        if increasing >= (nums.len() - faults - 1) {
            return true;
        }

        if decreasing >= (nums.len() - faults - 1) {
            return true;
        }

        return false;
    }

    fn parse_input(puzzle_input: String) -> Vec<Vec<i32>> {
        let lines: Result<Vec<_>, _> = puzzle_input
            .lines()
            .map(|l| {
                let report: Result<Vec<_>, _> =
                    l.split_whitespace().map(str::parse::<i32>).collect();

                report
            })
            .collect();

        lines.expect("Lines should contain valid i32 space-separated numbers.")
    }
}

impl Solution for MonotonicReport {
    fn part1(puzzle_input: String) -> String {
        MonotonicReport::parse_input(puzzle_input)
            .iter()
            .filter_map(|r| MonotonicReport::is_monotonic(r, 3, 0).then_some(1))
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        "".to_string()
    }
}
