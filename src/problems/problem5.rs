use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::{framework::*, util::transpose};

const SIZE: usize = 1000;
type BoardType = [[u64; SIZE]; SIZE];

fn new_board() -> Box<BoardType> {
    Box::new([[0; SIZE]; SIZE])
}

fn range(x: u64, y: u64) -> impl Iterator<Item = u64> {
    let d = (y as i64) - (x as i64);
    let sign = d / d.abs();
    (0_i64..=d.abs()).map(move |i| ((x as i64) + (i * sign)) as u64)
}

fn range_point(start: &Point, end: &Point) -> Vec<(u64, u64)> {
    if start.x == end.x {
        std::iter::repeat(start.x)
            .zip(range(start.y, end.y))
            .collect()
    } else if start.y == end.y {
        range(start.x, end.x)
            .zip(std::iter::repeat(start.y))
            .collect()
    } else {
        range(start.x, end.x).zip(range(start.y, end.y)).collect()
    }
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut coords = s.split(',');
        Self {
            x: coords.next().unwrap().parse::<u64>().unwrap(),
            y: coords.next().unwrap().parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn plot(&self, board: &mut Board) {
        range_point(&self.start, &self.end)
            .into_iter()
            .for_each(|(x, y)| {
                board.mark(x, y);
            })
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let mut points = s.split(" -> ");
        Self {
            start: points.next().unwrap().into(),
            end: points.next().unwrap().into(),
        }
    }
}

struct Board {
    board: Box<BoardType>,
}

impl Board {
    fn new() -> Self {
        Self { board: new_board() }
    }

    fn mark(&mut self, x: u64, y: u64) {
        self.board[y as usize][x as usize] += 1;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        const N: usize = 15;
        let s = self
            .board
            .iter()
            .take(N)
            .map(|row| row.iter().take(N).map(|v| v.to_string()).join(""))
            .join("\n");
        write!(f, "\n{}\n", s)
    }
}

pub struct Problem5 {}

impl Problem for Problem5 {
    type Output = u64;

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let lines: Vec<Line> = lines.map(|s| From::from(s.unwrap().as_str())).collect_vec();
        let non_diagonal = lines.into_iter().filter(|l| !l.is_diagonal());

        // i was trying to think of a way to do this with line intersections
        // e.g. by the rotation-order method...but two overlapping parallel lines...

        let mut board = Board::new();
        non_diagonal.for_each(|line| line.plot(&mut board));

        let mut list = Vec::new();
        board
            .board
            .iter()
            .enumerate()
            .map(move |(y, row)| row.iter().enumerate().map(move |(x, v)| (y, x, v)))
            .flatten()
            .filter(|(_y, _x, &v)| v > 1)
            .for_each(|v| {
                list.push(v);
            });

        list.len() as u64
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let lines: Vec<Line> = lines.map(|s| From::from(s.unwrap().as_str())).collect_vec();

        // i was trying to think of a way to do this with line intersections
        // e.g. by the rotation-order method...but two overlapping parallel lines...

        let mut board = Board::new();
        lines.iter().for_each(|line| line.plot(&mut board));

        let mut list = Vec::new();
        board
            .board
            .iter()
            .enumerate()
            .map(move |(y, row)| row.iter().enumerate().map(move |(x, v)| (y, x, v)))
            .flatten()
            .filter(|(_y, _x, &v)| v > 1)
            .for_each(|v| {
                list.push(v);
            });

        list.len() as u64
    }

    fn sample_answer_part1(&self) -> Self::Output {
        5
    }

    fn sample_answer_part2(&self) -> Self::Output {
        12
    }
}
