use crate::framework::Problem;
use std::fs::File;
use std::io;

pub struct Problem2 {}

impl Problem for Problem2 {
    type Output = u64;

    fn sample_answer_part1(&self) -> Self::Output {
        150
    }

    fn sample_answer_part2(&self) -> Self::Output {
        900
    }

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let (x, y) = lines.fold((0, 0), |acc, line| {
            let s = line
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let (direction, distance) = (&s[0], s[1].parse::<u64>().unwrap());
            match direction.as_str() {
                "forward" => (acc.0 + distance, acc.1),
                "down" => (acc.0, acc.1 + distance),
                "up" => (acc.0, acc.1 - distance),
                _ => panic!("Unsupported direction"),
            }
        });
        x * y
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let (x, y, _) = lines.fold((0, 0, 0), |acc, line| {
            let s = line
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let (direction, distance) = (&s[0], s[1].parse::<u64>().unwrap());
            match direction.as_str() {
                "forward" => (acc.0 + distance, acc.1 + distance * acc.2, acc.2),
                "down" => (acc.0, acc.1, acc.2 + distance),
                "up" => (acc.0, acc.1, acc.2 - distance),
                _ => panic!("Unsupported direction"),
            }
        });
        x * y
    }
}
