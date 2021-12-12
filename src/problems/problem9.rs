use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io;

use crate::framework::Problem;
use crate::util::get_neighbors_4d;

fn check_neighbors(grid: &[Vec<u64>], x: usize, y: usize) -> bool {
    let val = grid[y][x];
    get_neighbors_4d(x, y, (grid[0].len(), grid.len())).all(|(x, y)| grid[y][x] > val)
}

pub struct Problem9 {}

impl Problem for Problem9 {
    type Output = u64;

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let grid: Vec<Vec<u64>> = lines
            .map(|s| {
                s.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        let (xl, yl) = (grid[0].len(), grid.len());
        let indices = (0_usize..xl).flat_map(|x| (0_usize..yl).map(move |y| (x, y)));

        indices
            .map(|(x, y)| match check_neighbors(&grid, x, y) {
                true => grid[y][x] + 1,
                false => 0,
            })
            .sum()
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let grid: Vec<Vec<u64>> = lines
            .map(|s| {
                s.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        let (xl, yl) = (grid[0].len(), grid.len());
        let mut visited = (0..yl)
            .map(|_| (0..xl).map(|_| false).collect_vec())
            .collect_vec();
        let low_points = (0..xl)
            .flat_map(|x| (0..yl).map(move |y| (x, y)))
            .filter(|(x, y)| check_neighbors(&grid, *x, *y));

        fn visit(
            grid: &[Vec<u64>],
            visited: &mut [Vec<bool>],
            mut v: Vec<(usize, usize)>,
            x: usize,
            y: usize,
        ) -> Vec<(usize, usize)> {
            if visited[y][x] {
                return v;
            }

            visited[y][x] = true;
            match grid[y][x] {
                9 => {}
                _ => {
                    v.push((x, y));
                    let neighbors = get_neighbors_4d(x, y, (grid[0].len(), grid.len()))
                        .filter(|(x, y)| !visited[*y][*x])
                        .collect_vec(); // TODO: i don't want to collect his as a vec...
                    for (x, y) in neighbors {
                        v = visit(grid, visited, v, x, y);
                    }
                }
            }

            v
        }

        // iterate through low_points
        let mut basins = low_points
            .into_iter()
            .map(|(x, y)| visit(&grid, &mut visited, vec![], x, y))
            .collect_vec();
        basins.sort_by_key(|v| v.len());
        basins
            .iter()
            .rev()
            .take(3)
            .map(|v| v.len() as u64)
            .product()
    }

    fn sample_answer_part1(&self) -> Self::Output {
        15
    }

    fn sample_answer_part2(&self) -> Self::Output {
        1134
    }
}
