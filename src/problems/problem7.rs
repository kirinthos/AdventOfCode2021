use itertools::Itertools;

use crate::framework::Problem;

const SIZE: usize = 2000;
type Board = [u64; SIZE];

fn find_cost_1(board: Board, p: usize) -> u64 {
    board
        .iter()
        .enumerate()
        .map(|(position, count)| ((p as i32) - (position as i32)).abs() as u64 * count)
        .sum()
}

fn find_cost_2(board: Board, p: usize) -> u64 {
    board
        .iter()
        .enumerate()
        .map(|(position, count)| {
            let d = ((p as i32) - (position as i32)).abs() as u64;
            d * (d + 1) / 2 * count
        })
        .sum()
}

pub struct Problem7 {}

impl Problem for Problem7 {
    type Output = u64;

    fn solve_part1(
        &self,
        mut lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Self::Output {
        let line = lines.next().unwrap().unwrap();
        let crabs = line.split(',').map(|c| c.parse::<usize>().unwrap());

        let mut board = [0_u64; SIZE];
        for c in crabs {
            board[c] += 1;
        }

        (0..SIZE).map(|p| find_cost_1(board, p)).min().unwrap()
    }

    fn solve_part2(
        &self,
        mut lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Self::Output {
        let line = lines.next().unwrap().unwrap();
        let crabs = line.split(',').map(|c| c.parse::<usize>().unwrap());

        let mut board = [0_u64; SIZE];
        for c in crabs {
            board[c] += 1;
        }

        (0..SIZE).map(|p| find_cost_2(board, p)).min().unwrap()
    }

    fn sample_answer_part1(&self) -> Self::Output {
        37
    }

    fn sample_answer_part2(&self) -> Self::Output {
        168
    }
}
