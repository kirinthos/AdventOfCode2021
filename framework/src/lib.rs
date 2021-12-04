use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Problem {
    type Output;

    /// This function is called with the path to the input for problem part1
    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output;

    /// This function is called with the path to the input for problem part2
    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output;

    /// This function should return the answer to the sample input for part1
    fn sample_answer_part1(&self) -> Self::Output;

    /// This function should return the answer to the sample input for part2
    fn sample_answer_part2(&self) -> Self::Output;
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
