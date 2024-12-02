use std::path::Path;
use std::{env, fs};
use std::io;

use advent_of_code_2024::days::day1::ListSimilarity;
use advent_of_code_2024::solution::Solution;

fn get_test_input(day: u64) -> io::Result<String> {
    let s = format!("inputs/day{day}/test.txt");
    dbg!(&s);
    fs::read_to_string(Path::new(&s))
}

fn get_input(day: u64) -> io::Result<String> {
    let s = format!("inputs/day{day}/input.txt");
    fs::read_to_string(Path::new(&s))
}

fn run_solutions<T: Solution>(test_input: String, input: String) {
    println!("Tests");
    let test_answer1 = T::part1(test_input.clone());
    println!("{test_answer1}");

    let test_answer2 = T::part2(test_input);
    println!("{test_answer2}");

    println!("Solutions");
    let answer1 = T::part1(input.clone());
    println!("{answer1}");

    let answer2 = T::part2(input);
    println!("{answer2}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u64 = args[1].parse().unwrap();

    let test_input = get_test_input(day).expect("File is read correctly");
    let input = get_input(day).expect("File is read correctly");

    match day {
        1 => run_solutions::<ListSimilarity>(test_input, input),
        _ => println!("No day solution for day {day}")
    }
}
