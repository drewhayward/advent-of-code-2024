use std::collections::HashMap;

use crate::solution::Solution;

pub struct ListSimilarity;

impl ListSimilarity {
    fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
        input
            .lines()
            .map(|l| {
                let a: Vec<_> = l
                    .split("   ")
                    .map(|v| v.parse::<i32>().unwrap())
                    .take(2)
                    .collect();
                (a[0], a[1])
            })
            .unzip()
    }
}

impl Solution for ListSimilarity {
    fn part1(puzzle_input: String) -> String {
        let (mut list1, mut list2) = ListSimilarity::parse_input(puzzle_input);

        list1.sort();
        list2.sort();

        let diff = list1
            .iter()
            .zip(list2)
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .to_string();

        diff
    }

    fn part2(puzzle_input: String) -> String {
        let (list1, list2) = ListSimilarity::parse_input(puzzle_input);

        // Build a hash map of counts in list 2
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in list2 {
            let current = counts.get(&num).unwrap_or(&0);
            counts.insert(num, current + 1);
        }


        list1
            .iter()
            .map(|n| n * counts.get(&n).unwrap_or(&0) )
            .sum::<i32>()
            .to_string()
    }
}
