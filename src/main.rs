use std::path::Path;
use std::{env, fs};
use std::io;

use advent_of_code_2024::days::bridge_repair::BridgeSolution;
use advent_of_code_2024::days::chronospatial_comp::ChronospatialSolution;
use advent_of_code_2024::days::claw::ClawContraption;
use advent_of_code_2024::days::day1::ListSimilarity;
use advent_of_code_2024::days::disk::DiskFragmenter;
use advent_of_code_2024::days::garden::GardenGroupSolution;
use advent_of_code_2024::days::guard::GuardSolution;
use advent_of_code_2024::days::hoof::HoofItSolution;
use advent_of_code_2024::days::monotonic::MonotonicReport;
use advent_of_code_2024::days::mull::MullSolution;
use advent_of_code_2024::days::pebbles::PebbleCounterSolution;
use advent_of_code_2024::days::print_order::PrintOrder;
use advent_of_code_2024::days::reindeer_maze::ReindeerSolution;
use advent_of_code_2024::days::resonant::Resonant;
use advent_of_code_2024::days::restroom::RestroomSolution;
use advent_of_code_2024::days::xmas::XmasSearchSolution;
use advent_of_code_2024::solution::Solution;

fn get_test_input(day: u64) -> io::Result<String> {
    let s = format!("inputs/day{day}/test.txt");
    fs::read_to_string(Path::new(&s))
}

fn get_input(day: u64) -> io::Result<String> {
    let s = format!("inputs/day{day}/input.txt");
    fs::read_to_string(Path::new(&s))
}

fn run_solutions<T: Solution>(test_input: String, input: String) {
    //println!("Tests");
    //let test_answer1 = T::part1(test_input.clone());
    //println!("{test_answer1}");
    //
    //let test_answer2 = T::part2(test_input);
    //println!("{test_answer2}");
    //
    //println!("Solutions");
    //let answer1 = T::part1(input.clone());
    //println!("{answer1}");

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
        2 => run_solutions::<MonotonicReport>(test_input, input),
        3 => run_solutions::<MullSolution>(test_input, input),
        4 => run_solutions::<XmasSearchSolution>(test_input, input),
        5 => run_solutions::<PrintOrder>(test_input, input),
        6 => run_solutions::<GuardSolution>(test_input, input),
        7 => run_solutions::<BridgeSolution>(test_input, input),
        8 => run_solutions::<Resonant>(test_input, input),
        9 => run_solutions::<DiskFragmenter>(test_input, input),
        10 => run_solutions::<HoofItSolution>(test_input, input),
        11 => run_solutions::<PebbleCounterSolution>(test_input, input),
        12 => run_solutions::<GardenGroupSolution>(test_input, input),
        13 => run_solutions::<ClawContraption>(test_input, input),
        14 => run_solutions::<RestroomSolution>(test_input, input),
        16 => run_solutions::<ReindeerSolution>(test_input, input),
        17 => run_solutions::<ChronospatialSolution>(test_input, input),
        _ => println!("No day solution for day {day}")
    }
}
