use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io;

use crate::framework::Problem;

static SYNTAX_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    std::array::IntoIter::new([('{', '}'), ('(', ')'), ('[', ']'), ('<', '>')]).collect()
});

static POINT_MAP: Lazy<HashMap<char, u64>> = Lazy::new(|| {
    std::array::IntoIter::new([('}', 1197), (')', 3), (']', 57), ('>', 25137)]).collect()
});

static POINT_MAP_PART_2: Lazy<HashMap<char, u64>> =
    Lazy::new(|| std::array::IntoIter::new([(')', 1), (']', 2), ('}', 3), ('>', 4)]).collect());

pub struct Problem10 {}

impl Problem for Problem10 {
    type Output = u64;

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        lines
            .map(|s| {
                let mut v = vec![];
                let mut pts = 0;
                for c in s.unwrap().chars() {
                    match SYNTAX_MAP.get(&c) {
                        Some(m) => v.push(m),
                        None => {
                            let req = v.pop().unwrap();
                            if c != *req {
                                pts = POINT_MAP[&c];
                                break;
                            }
                        }
                    }
                }
                pts
            })
            .sum()
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let mut scores = lines
            .filter_map(|s| {
                let mut v = vec![];
                for c in s.unwrap().chars() {
                    match SYNTAX_MAP.get(&c) {
                        Some(m) => v.push(m),
                        None => {
                            let req = v.pop().unwrap();
                            if c != *req {
                                return None;
                            }
                        }
                    }
                }
                Some(
                    v.into_iter()
                        .rev()
                        .fold(0, |acc, n| acc * 5 + POINT_MAP_PART_2[n]),
                )
            })
            .collect_vec();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }

    fn sample_answer_part1(&self) -> Self::Output {
        26397
    }

    fn sample_answer_part2(&self) -> Self::Output {
        288957
    }
}
