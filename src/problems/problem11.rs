use itertools::Itertools;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io;

use crate::{
    framework::Problem,
    util::{get_neighbors_8d, print_board},
};

fn explode(board: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u64 {
    if board[y][x] < 10 {
        return 0;
    }

    board[y][x] = 0;

    let mut s = 1;
    for (x, y) in get_neighbors_8d(x, y, (board[0].len(), board.len())) {
        if board[y][x] == 0 {
            continue;
        }
        board[y][x] += 1;
        s += explode(board, x, y);
    }
    s
}

pub struct Problem11 {}

impl Problem for Problem11 {
    type Output = u64;

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let mut board = lines
            .map(|s| {
                s.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let mut flashes = 0;
        for _ in 0..100 {
            // increment everyone by one
            let flashers = board
                .iter_mut()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter_mut().enumerate().filter_map(move |(x, v)| {
                        *v += 1;
                        (*v > 9).then(move || (x, y))
                    })
                })
                .collect_vec();
            // flash everyone
            for (x, y) in flashers.into_iter() {
                flashes += explode(&mut board, x, y);
            }

            for (x, y) in (0..board[0].len()).cartesian_product(0..board.len()) {
                if board[y][x] > 9 {
                    board[y][x] = 0;
                }
            }
        }

        flashes
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let mut board = lines
            .map(|s| {
                s.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect_vec()
            })
            .collect_vec();

        for i in 1.. {
            // increment everyone by one
            let flashers = board
                .iter_mut()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter_mut().enumerate().filter_map(move |(x, v)| {
                        *v += 1;
                        (*v > 9).then(move || (x, y))
                    })
                })
                .collect_vec();
            // flash everyone
            for (x, y) in flashers.into_iter() {
                explode(&mut board, x, y);
            }

            for (x, y) in (0..board[0].len()).cartesian_product(0..board.len()) {
                if board[y][x] > 9 {
                    board[y][x] = 0;
                }
            }

            if board.iter().flat_map(|row| row.iter()).sum::<u32>() == 0 {
                return i;
            }
        }
        0
    }

    fn sample_answer_part1(&self) -> Self::Output {
        1656
    }

    fn sample_answer_part2(&self) -> Self::Output {
        195
    }
}
