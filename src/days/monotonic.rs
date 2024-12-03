use core::panic;
use std::collections::HashSet;

use crate::solution::Solution;

pub struct MonotonicReport;

struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Interval {
        Interval { start, end }
    }

    fn direction(&self) -> Direction {
        match self.end - self.start {
            x if x > 0 => Direction::Increasing,
            x if x < 0 => Direction::Decreasing,
            _ => Direction::Neither,
        }
    }

    fn is_monotonic(&self, direction: &Direction) -> bool {
        self.direction() == *direction
    }

    fn is_within_tolerance(&self, tolerance: i32) -> bool {
        (self.start - self.end).abs() <= tolerance
    }

    //fn is_good(&self, direction: &Direction, tolerance: i32) -> bool {
    //    self.is_monotonic(direction) && self.is_within_tolerance(tolerance)
    //}
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
    Neither,
}

impl MonotonicReport {
    fn list_direction(nums: &[i32]) -> Direction {
        let mut increasing = 0;
        let mut decreasing = 0;

        for window in nums.windows(2) {
            match window[1] - window[0] {
                x if x > 0 => increasing += 1,
                x if x < 0 => decreasing += 1,
                _ => (),
            }
        }

        match (increasing, decreasing) {
            (i, d) if i > d => Direction::Increasing,
            (i, d) if d > i => Direction::Decreasing,
            _ => Direction::Neither,
        }
    }

    fn is_monotonic(nums: &[i32], tolerance: i32) -> bool {
        let list_dir = MonotonicReport::list_direction(nums);
        return nums
            .windows(2)
            .map(|window| Interval::new(window[0], window[1]))
            .all(|i| i.is_monotonic(&list_dir) && i.is_within_tolerance(tolerance));
    }

    fn is_monotonic_with_fault(nums: &[i32], tolerance: i32) -> bool {
        if MonotonicReport::is_monotonic(nums, tolerance) {
            return true;
        }

        let list_dir = MonotonicReport::list_direction(nums);
        if list_dir == Direction::Neither {
            return false;
        }

        for (i, _) in nums.iter().enumerate() {
            let new_nums = [&nums[..i], &nums[i + 1..]].concat();

            if MonotonicReport::is_monotonic(&new_nums, tolerance) {
                return true;
            }
        }

        false
    }

    //fn is_monotonic(nums: &[i32], tolerance: i32, allow_fault: bool) -> bool {
    //    let list_dir = MonotonicReport::list_direction(nums);
    //
    //    if !allow_fault {
    //        return nums
    //            .windows(2)
    //            .map(|window| Interval::new(window[0], window[1]))
    //            .all(|i| i.is_good(&list_dir, tolerance));
    //    }
    //
    //    let mut faults: HashSet<usize> = HashSet::new();
    //    for (i, window) in nums.windows(3).enumerate() {
    //        let interval1 = Interval::new(window[0], window[1]);
    //        let interval2 = Interval::new(window[1], window[2]);
    //        let interval3 = Interval::new(window[0], window[2]);
    //
    //        match (
    //            interval1.is_good(&list_dir, tolerance),
    //            interval2.is_good(&list_dir, tolerance),
    //            interval3.is_good(&list_dir, tolerance),
    //        ) {
    //            // no faults detected
    //            (true, true, _) => continue,
    //            // Fault recovery
    //            (_, _, true) => faults.insert(i + 1), // fault at win[1]
    //            (false, true, false) => faults.insert(i + 0), // fault at win[0]
    //            (true, false, false) => faults.insert(i + 2), // fault at win[2]
    //            (false, false, false) => return false,
    //        };
    //    }
    //
    //    dbg!(&faults);
    //    faults.len() <= 1
    //}

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
            .filter_map(|r| MonotonicReport::is_monotonic(r, 3).then_some(1))
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        MonotonicReport::parse_input(puzzle_input)
            .iter()
            .filter_map(|r| MonotonicReport::is_monotonic_with_fault(r, 3).then_some(1))
            .count()
            .to_string()
    }
}

mod test {
    use crate::days::monotonic::Direction;

    use super::MonotonicReport;

    // Basic monotonic tests
    #[test]
    fn detects_increasing() {
        assert!(MonotonicReport::is_monotonic(&vec![1, 2, 3, 4, 5], 1,));
    }

    #[test]
    fn detects_decreasing() {
        assert!(MonotonicReport::is_monotonic(&vec![5, 4, 3, 2, 1], 1,));
    }

    #[test]
    fn fails_over_tolerance_increasing() {
        assert!(!MonotonicReport::is_monotonic(&vec![1, 2, 3, 1, 4], 1,));
    }

    #[test]
    fn fails_over_tolerance_decreasing() {
        assert!(!MonotonicReport::is_monotonic(&vec![10, 8, 10, 7, 6], 1,));
    }

    // Fault tests without fault allowance
    #[test]
    fn panics_faults_desc() {
        assert!(!MonotonicReport::is_monotonic(
            &vec![5, 4, 100, 2, 1],
            2,
        ));
    }

    #[test]
    fn panics_faults_asc() {
        assert!(!MonotonicReport::is_monotonic(
            &vec![1, 2, 3, -100, 4, 5],
            2,
        ));
    }

    // Fault tests with fault allowance
    #[test]
    fn handles_faults_desc() {
        assert!(MonotonicReport::is_monotonic_with_fault(
            &vec![5, 4, 100, 2, 1],
            2,
        ));
    }

    #[test]
    fn handles_faults_asc() {
        assert!(MonotonicReport::is_monotonic_with_fault(
            &vec![1, 2, 3, -100, 4, 5],
            2,
        ));
    }

    // Edge cases
    #[test]
    fn fails_multiple_faults() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 2, -100, 4, -100, 6],
            2,
        ));
    }

    #[test]
    fn handles_fault_at_end() {
        assert!(MonotonicReport::is_monotonic_with_fault(&vec![1, 2, 3, 4, 0], 2));
    }

    #[test]
    fn handles_fault_at_start() {
        assert!(MonotonicReport::is_monotonic_with_fault(
            &vec![100, 1, 2, 3, 4],
            2,
        ));
    }

    #[test]
    fn handles_fault_at_start_subtle() {
        assert!(MonotonicReport::is_monotonic_with_fault(
            &vec![1, 4, 5, 6, 7, 8],
            1,
        ));
    }

    #[test]
    fn it_is_increasing() {
        assert!(MonotonicReport::list_direction(&vec![100, 1, 2, 3, 4]) == Direction::Increasing)
    }
    

    #[test]
    fn fails_zigzag() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 3, 2, 4, 3],
            1
        ));
    }

    #[test]
    fn fails_double_peak() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 5, 2, 5, 3],
            2
        ));
    }

    #[test]
    fn fails_valley_pattern() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![5, 1, 4, 2, 5],
            2
        ));
    }

    #[test]
    fn fails_alternating() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 4, 2, 5, 3],
            1
        ));
    }

    #[test]
    fn fails_triple_fault() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 5, 2, 6, 3],
            2
        ));
    }


    #[test]
    fn fails_long_zigzag() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 5, 2, 6, 3, 7, 4],
            2
        ));
    }

    #[test]
    fn fails_subtle_zigzag() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 3, 2, 4, 3, 5, 4],
            1
        ));
    }

    #[test]
    fn fails_plateau_with_fault() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 4, 4, 4, 2, 5],
            1
        ));
    }

    #[test]
    fn fails_almost_monotonic() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 4, 2, 5, 3, 6, 4],
            1
        ));
    }

    #[test]
    fn fails_two_close_faults() {
        assert!(!MonotonicReport::is_monotonic_with_fault(
            &vec![1, 3, 2, 4, 5],
            2
        ));
    }
}
