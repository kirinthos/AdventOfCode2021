#![cfg(test)]
use itertools::Itertools;

static SAMPLE: &str = include_str!("../input.sample");
static PROBLEM: &str = include_str!("../input.problem");

fn solve_part1(values: Vec<i32>) -> i32 {
    values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}

fn solve_part2(values: Vec<i32>) -> i32 {
    solve_part1(
        values
            .iter()
            .tuple_windows::<(_, _, _)>()
            .map(|l| l.0 + l.1 + l.2)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_part1_sample() {
        let values: Vec<_> = SAMPLE
            .split('\n')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        assert_eq!(solve_part1(values), 7);
    }

    #[test]
    fn problem1_part1_problem() {
        let values: Vec<_> = PROBLEM
            .split('\n')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        panic!("answer: {}", solve_part1(values));
    }

    #[test]
    fn problem1_part2_sample() {
        let values: Vec<_> = SAMPLE
            .split('\n')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        assert_eq!(solve_part2(values), 5);
    }

    #[test]
    fn problem1_part2_problem() {
        let values: Vec<_> = PROBLEM
            .split('\n')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        panic!("answer: {}", solve_part2(values));
    }
}
