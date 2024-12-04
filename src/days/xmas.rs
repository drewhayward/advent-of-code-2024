use std::{char, ops::Add, usize};

use crate::solution::Solution;

const DIRS: [Position; 8] = [
    Position { x: 0, y: 1 },
    Position { x: 1, y: 0 },
    Position { x: 0, y: -1 },
    Position { x: -1, y: 0 },
    Position { x: 1, y: 1 },
    Position { x: -1, y: 1 },
    Position { x: 1, y: -1 },
    Position { x: -1, y: -1 },
];

type WordSearch = Vec<Vec<char>>;

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Defines an iterator which handles bounds checking on a search board
#[derive(Clone)]
struct SearchIter<'a> {
    search: &'a WordSearch,
    position: Position,
    direction: Position,
    steps: u32,
    length: u32,
}

impl<'a> Iterator for SearchIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.y < 0
            || self.position.y >= self.search.len() as i64
            || self.position.x < 0
            || self.position.x >= self.search[0].len() as i64
            || self.length <= self.steps
        {
            return None;
        }

        let val = Some(self.search[self.position.y as usize][self.position.x as usize]);

        self.position = self.position + self.direction;
        self.steps += 1;

        val
    }
}

pub struct XmasSearchSolution;

impl XmasSearchSolution {
    fn parse_input(puzzle_input: String) -> WordSearch {
        return puzzle_input
            .lines()
            .map(|l| Vec::from_iter(l.chars()))
            .collect();
    }

    fn build_iterators<'a>(search: &'a WordSearch, position: &Position) -> Vec<SearchIter<'a>> {
        DIRS.map(|direction| SearchIter {
            search: &search,
            length: 4,
            steps: 0,
            position: *position,
            direction,
        })
        .to_vec()
    }
}

impl Solution for XmasSearchSolution {
    fn part1(puzzle_input: String) -> String {
        let search = XmasSearchSolution::parse_input(puzzle_input);

        // Build
        let mut iterators: Vec<SearchIter> = Vec::new();
        for (y, row) in search.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if *char != 'X' {
                    continue;
                };

                iterators.append(&mut XmasSearchSolution::build_iterators(
                    &search,
                    &Position {
                        x: x as i64,
                        y: y as i64,
                    },
                ));
            }
        }

        iterators
            .iter_mut()
            .filter_map(|s| s.eq("XMAS".chars()).then_some(1))
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let search = XmasSearchSolution::parse_input(puzzle_input);

        let mut total = 0;
        for (y, row) in search.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                // Define 2 different diagonal iterators and compare them to MAS & SAM
                let diag1 = SearchIter {
                    search: &search,
                    position: Position {
                        x: x as i64,
                        y: y as i64,
                    },
                    direction: Position { x: 1, y: 1 },
                    length: 3,
                    steps: 0,
                };

                let diag2 = SearchIter {
                    search: &search,
                    position: Position {
                        x: (x + 2) as i64,
                        y: y as i64,
                    },
                    direction: Position { x: -1, y: 1 },
                    length: 3,
                    steps: 0,
                };

                if (diag1.clone().eq("MAS".chars()) || diag1.clone().eq("SAM".chars()))
                    && (diag2.clone().eq("MAS".chars()) || diag2.clone().eq("SAM".chars()))
                {
                    total += 1
                }
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let search = XmasSearchSolution::parse_input(
            "abcdef
aBcdef
abCdef
abcDef
abcdef
abcdef"
                .to_string(),
        );

        // Right
        let mut search_iter = SearchIter {
            search: &search,
            position: Position { x: 0, y: 0 },
            direction: Position { x: 1, y: 0 },
            length: 4,
            steps: 0,
        };

        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), Some('b'));
        assert_eq!(search_iter.next(), Some('c'));
        assert_eq!(search_iter.next(), Some('d'));
        assert_eq!(search_iter.next(), None);

        // Down
        search_iter = SearchIter {
            search: &search,
            position: Position { x: 0, y: 0 },
            direction: Position { x: 0, y: 1 },
            length: 4,
            steps: 0,
        };

        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), None);

        // Diagonal right and down
        search_iter = SearchIter {
            search: &search,
            position: Position { x: 0, y: 0 },
            direction: Position { x: 1, y: 1 },
            length: 4,
            steps: 0,
        };

        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), Some('B'));
        assert_eq!(search_iter.next(), Some('C'));
        assert_eq!(search_iter.next(), Some('D'));
        assert_eq!(search_iter.next(), None);

        // Out of bounds
        search_iter = SearchIter {
            search: &search,
            position: Position { x: 0, y: 0 },
            direction: Position { x: -1, y: 0 },
            length: 4,
            steps: 0,
        };

        assert_eq!(search_iter.next(), Some('a'));
        assert_eq!(search_iter.next(), None);
    }
}
