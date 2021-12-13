use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fs::File;
use std::io;

use crate::framework::Problem;

// hmm, i hope we don't do arbitrary lines
#[derive(Debug)]
enum Reflect {
    X(i32),
    Y(i32),
}

fn reflect(board: HashSet<(i32, i32)>, r: Reflect) -> HashSet<(i32, i32)> {
    board
        .into_iter()
        .map(|(x, y)| match r {
            Reflect::X(dx) if x > dx => (2 * dx - x, y),
            Reflect::Y(dy) if y > dy => (x, 2 * dy - y),
            _ => (x, y),
        })
        .collect()
}

pub struct Problem13 {}

impl Problem for Problem13 {
    type Output = u64;

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let lines = lines.map(|s| s.unwrap()).collect_vec();
        let (split_pos, _) = lines.iter().find_position(|l| l.is_empty()).unwrap();
        let (board, folds) = (&lines[..split_pos], &lines[split_pos + 1..]);

        let board: HashSet<(i32, i32)> = board
            .iter()
            .map(|l| {
                let mut i = l.split(',');
                (
                    i.next().unwrap().parse().unwrap(),
                    i.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        let folds = folds
            .iter()
            .map(|l| {
                let mut i = l.split(' ').nth(2).unwrap().split('=');
                match i.next().unwrap() {
                    "x" => Reflect::X(i.next().unwrap().parse().unwrap()),
                    "y" => Reflect::Y(i.next().unwrap().parse().unwrap()),
                    _ => panic!("bye"),
                }
            })
            .collect_vec();

        folds.into_iter().take(1).fold(board, reflect).len() as u64
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let lines = lines.map(|s| s.unwrap()).collect_vec();
        let (split_pos, _) = lines.iter().find_position(|l| l.is_empty()).unwrap();
        let (board, folds) = (&lines[..split_pos], &lines[split_pos + 1..]);

        let board: HashSet<(i32, i32)> = board
            .iter()
            .map(|l| {
                let mut i = l.split(',');
                (
                    i.next().unwrap().parse().unwrap(),
                    i.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        let folds = folds
            .iter()
            .map(|l| {
                let mut i = l.split(' ').nth(2).unwrap().split('=');
                match i.next().unwrap() {
                    "x" => Reflect::X(i.next().unwrap().parse().unwrap()),
                    "y" => Reflect::Y(i.next().unwrap().parse().unwrap()),
                    _ => panic!("bye"),
                }
            })
            .collect_vec();

        let board = folds.into_iter().fold(board, reflect);

        let (mx, my): (i32, i32) = board
            .iter()
            .fold((0, 0), |acc, n| (acc.0.max(n.0), acc.1.max(n.1)));

        for y in 0..(my + 5) {
            for x in 0..(mx + 5) {
                print!(
                    "{}",
                    match board.contains(&(x, y)) {
                        true => "#",
                        false => ".",
                    }
                );
            }
            println!();
        }

        board.len() as u64
    }

    fn sample_answer_part1(&self) -> Self::Output {
        17
    }

    // TODO: oh okay there's no sample answer to part two....fix this later with Option
    fn sample_answer_part2(&self) -> Self::Output {
        16
    }
}
