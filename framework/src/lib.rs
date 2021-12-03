use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Problem {
    type Output;

    fn solve(&self, filepath: &str) -> Self::Output;
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
