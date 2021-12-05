use crate::framework::Problem;
use std::fs::File;
use std::io;

// Note: i did this as lists because i'm so used to parsing things that way
// but realistically this should just be a list of numbers and then we can
// use masks to sum bits....

struct BitCounts {
    bit_counts: Vec<u64>,
    half_length: u64,
    mask: u64,
}

fn lines_to_bits(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u64>> {
    lines
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect()
}

fn bits_to_bit_counts(bits: Vec<Vec<u64>>) -> BitCounts {
    let half_length: u64 = (bits.len() as u64 + 1) / 2;
    // mask to extract the correct amount of bits from a flip
    let mask = 2_u64.pow(bits[0].len() as u32) - 1;

    BitCounts {
        bit_counts: bits
            .into_iter()
            .reduce(|acc, n| acc.iter().zip(n.iter()).map(|(a, b)| a + b).collect())
            .unwrap(),
        half_length,
        mask,
    }
}

fn bit_list_to_u64(bits: Vec<u64>) -> u64 {
    bits.into_iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if b == 1 { 2_u64.pow(i as u32) } else { 0 })
        .sum()
}

fn part2(bits: Vec<Vec<u64>>, digit: usize, most_significant: bool) -> u64 {
    let l = (bits.len() as u64 + 1) / 2;
    let digit_bits: Vec<u64> = bits.iter().map(|b| *b.get(digit).unwrap()).collect();
    let sum = digit_bits.iter().sum::<u64>();
    let check = sum >= l;
    let check = if !most_significant { !check } else { check } as u64;
    let mut remaining: Vec<Vec<_>> = bits
        .into_iter()
        .zip(digit_bits.into_iter())
        .filter(|(_bits, digit_bit)| *digit_bit == check)
        .map(|p| p.0)
        .collect();

    match remaining.len() {
        1 => bit_list_to_u64(remaining.pop().unwrap()),
        _ => part2(remaining, digit + 1, most_significant),
    }
}

pub struct Problem3 {}

impl Problem for Problem3 {
    type Output = u64;

    fn sample_answer_part1(&self) -> Self::Output {
        198
    }

    fn sample_answer_part2(&self) -> Self::Output {
        230
    }

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let bits = lines_to_bits(lines);

        let BitCounts {
            bit_counts,
            half_length,
            mask,
        } = bits_to_bit_counts(bits);
        let n: u64 = bit_list_to_u64(
            bit_counts
                .into_iter()
                .map(|bc| (bc >= half_length) as u64)
                .collect(),
        );

        n * (!n & mask)
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let bits = lines_to_bits(lines);

        let n1 = part2(bits.clone(), 0, true);
        let n2 = part2(bits, 0, false);
        n1 * n2
    }
}
