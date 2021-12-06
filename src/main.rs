#![allow(unused_imports)]
use std::collections::HashMap;

use clap::{App, Arg};
use once_cell::sync::Lazy;

pub mod framework;
pub mod problems;
pub mod util;

use crate::framework::*;
use crate::problems::*;

const DATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data");
static PROBLEM_MAP: Lazy<HashMap<u32, ProblemType>> = Lazy::new(|| {
    // TODO: include problem 1
    (2_u32..)
        .zip(std::array::IntoIter::new([
            ProblemType::NonNegative(Box::new(Problem2 {})),
            ProblemType::NonNegative(Box::new(Problem3 {})),
            ProblemType::NonNegative(Box::new(Problem4 {})),
            ProblemType::NonNegative(Box::new(Problem5 {})),
            ProblemType::NonNegative(Box::new(Problem6 {})),
        ]))
        .collect()
});

fn main() {
    let matches = App::new("Advent of Code 2021 Solver")
        .version("0.1")
        .author("Jay Cunningham <jeremiah.w.cunningham@gmail.com>")
        .about("Simple solution framework for the Advent of Code series")
        .arg(
            Arg::with_name("problem_number")
                .short("n")
                .long("problem")
                .value_name("PROBLEM")
                .help("Choose which problem to solve")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part2")
                .help("When set, solve part 2"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let problem_number = matches
        .value_of("problem_number")
        .expect("provide a problem choice")
        .parse::<u32>()
        .expect("valid number");

    let part = matches.is_present("part");

    let problem = PROBLEM_MAP
        .get(&problem_number)
        .expect("pick a valid problem number!");

    let problem = match problem {
        ProblemType::NonNegative(problem) => problem,
    };

    let sample_path = format!("{}/sample.{}", DATA_DIR, problem_number);
    let input_path = format!("{}/input.{}", DATA_DIR, problem_number);
    let sample = read_lines(sample_path).unwrap();
    let input = read_lines(input_path).unwrap();

    match part {
        false => {
            assert_eq!(problem.solve_part1(sample), problem.sample_answer_part1());
            println!("Part 1: {}", problem.solve_part1(input));
        }
        true => {
            assert_eq!(problem.solve_part2(sample), problem.sample_answer_part2());
            println!("Part 2: {}", problem.solve_part2(input));
        }
    }
}
