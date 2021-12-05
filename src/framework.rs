pub use std::fs::File;
pub use std::io::{self, BufRead};
pub use std::path::Path;

pub enum ProblemType {
    NonNegative(Box<dyn Problem<Output = u64> + Send + Sync>),
}

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
