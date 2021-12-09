use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::framework::Problem;

// idk it was late and i was having trouble using sets, though with strings so small
// set is probably slower
fn contains_chars(haystack: &str, needle: &str, m: usize) -> bool {
    needle
        .chars()
        .map(|c| haystack.contains(c) as usize)
        .sum::<usize>()
        == m
}

pub struct Problem8 {}

impl Problem for Problem8 {
    type Output = u64;

    fn solve_part1(
        &self,
        lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Self::Output {
        lines
            .map(|s| s.unwrap())
            .map(|s| {
                let mut parts = s.split(" | ");
                let (_puzzle, code) = (parts.next().unwrap(), parts.next().unwrap());
                code.split(' ')
                    .map(|c| match c.len() {
                        2 | 3 | 4 | 7 => 1_u64,
                        _ => 0_u64,
                    })
                    .collect_vec()
            })
            .flatten()
            .sum()
    }

    fn solve_part2(
        &self,
        lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Self::Output {
        lines
            .map(|s| s.unwrap().replace("| ", ""))
            .map(|s| {
                let digits = s
                    .split(' ')
                    .map(|s| s.chars().sorted().collect::<String>())
                    .collect_vec();
                let code = digits
                    .iter()
                    .map(|s| s.as_str())
                    .skip(digits.len() - 4)
                    .collect_vec();
                // ...i'd love to not do this in this way, but whatever i guess...
                // can determine 1, 4, 7, 8 digits
                let (one, remaining_digits) = digits
                    .iter()
                    .map(|s| s.as_str())
                    .partition::<Vec<&str>, _>(|s| s.len() == 2);
                let (four, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|&&s| s.len() == 4);
                let (seven, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|&&s| s.len() == 3);
                let (eight, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|&&s| s.len() == 7);

                // if it contains 4's parts, it must be 9
                let (nine, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|&&s| contains_chars(s, four[0], 4));
                // if it contains 6 of 8's parts, and all of 7's, it must be 0
                let (zero, remaining_digits) =
                    remaining_digits.iter().partition::<Vec<&str>, _>(|&&s| {
                        contains_chars(s, eight[0], 6) && contains_chars(s, seven[0], 3)
                    });
                // if it contains 6 of 8's parts, it must be six
                let (six, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|&&s| contains_chars(s, eight[0], 6));
                // if it contains 5 of 6's parts, it must be 5
                let (five, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|&&s| contains_chars(s, six[0], 5));
                // if it contains 7's parts, it must be 3
                let (three, remaining_digits) = remaining_digits
                    .iter()
                    .partition::<Vec<&str>, _>(|s| contains_chars(s, seven[0], 3));
                // it's 2
                let two = remaining_digits;

                let map: HashMap<_, _> = std::array::IntoIter::new([
                    *zero.get(0).unwrap_or(&""),
                    *one.get(0).unwrap_or(&""),
                    *two.get(0).unwrap_or(&""),
                    *three.get(0).unwrap_or(&""),
                    *four.get(0).unwrap_or(&""),
                    *five.get(0).unwrap_or(&""),
                    *six.get(0).unwrap_or(&""),
                    *seven.get(0).unwrap_or(&""),
                    *eight.get(0).unwrap_or(&""),
                    *nine.get(0).unwrap_or(&""),
                ])
                .enumerate()
                .map(|(i, v)| (v, i as u64))
                .collect();

                code.into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, s)| map[s] * 10_u64.pow(i as u32))
                    .sum::<u64>()
            })
            .sum()
    }

    fn sample_answer_part1(&self) -> Self::Output {
        26
    }

    fn sample_answer_part2(&self) -> Self::Output {
        61229
    }
}
