use itertools::Itertools;
use std::hash::Hash;
use std::{collections::HashSet, convert::TryInto};

pub fn has_unique_elements<T, I>(mut iter: I) -> bool
where
    T: Hash + Eq,
    I: Iterator<Item = T> + std::fmt::Debug,
{
    let mut set = HashSet::new();
    iter.all(move |v| set.insert(v))
}

pub fn print_board<T: std::fmt::Display>(v: &[Vec<T>], separator: &str) {
    let p = v.iter().map(|row| row.iter().join(separator)).join("\n");
    println!("{}\n", p);
}

pub fn get_neighbors_4d(
    x: usize,
    y: usize,
    size: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    get_neighbors(x, y, size, &DIRS)
}

pub fn get_neighbors_8d(
    x: usize,
    y: usize,
    size: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    get_neighbors(x, y, size, &DIRS)
}

pub fn get_neighbors(
    x: usize,
    y: usize,
    (xl, yl): (usize, usize),
    dirs: &'static [(i32, i32)],
) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = (x as i32, y as i32);
    dirs.iter().filter_map(move |(dx, dy)| {
        TryInto::<usize>::try_into(y + dy)
            .ok()
            .and_then(|y| TryInto::<usize>::try_into(x + dx).ok().map(|x| (x, y)))
            .and_then(|(x, y)| (x < xl && y < yl).then(|| (x, y)))
    })
}

pub fn transpose<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_basic() {
        let v = transpose(&[vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        let r = vec![vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8]];

        assert_eq!(v, r);
    }
}
