use itertools::Itertools;

use crate::{framework::*, util::transpose};

#[derive(Debug)]
struct BingoBoard {
    size: (usize, usize),
    card: Vec<Vec<u64>>,
    matched: Vec<Vec<bool>>,
}

impl BingoBoard {
    /// Returns true if the victory condition is met! (a row or column is complete)
    pub fn check_victory(&self) -> bool {
        self.matched.iter().any(|row| row.iter().all(|&b| b))
            || transpose(&self.matched)
                .iter()
                .any(|row| row.iter().all(|&b| b))
    }

    /// Marks a space on the card, returns true if we've won
    pub fn mark(&mut self, number: u64) -> bool {
        if let Some((i, _)) = self
            .card
            .iter()
            .flatten()
            .enumerate()
            .find(|(_, &v)| v == number)
        {
            let (y, x) = (i / self.size.0, i % self.size.1);
            self.matched[y][x] = true;
        }
        self.check_victory()
    }

    /// Finds the sum of unmarked squares
    pub fn unmatched_sum(&self) -> u64 {
        self.card
            .iter()
            .flatten()
            .zip(self.matched.iter().flatten())
            .map(|(&c, &m)| if !m { c } else { 0 })
            .sum()
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s = self
            .card
            .iter()
            .map(|row| row.iter().map(|v| format!("{:03}", v)).join(" "))
            .join("\n");
        write!(f, "{}", s)
    }
}

impl<S: Into<String>, I: Iterator<Item = S>> From<I> for BingoBoard {
    fn from(iter: I) -> Self {
        let card = iter
            .filter_map(|line| match line.into().as_str() {
                "" => None,
                v => Some(
                    v.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect(),
                ),
            })
            .collect();
        // TODO: dynamic size? this was just faster to write
        let matched = [[false; 5]; 5].iter().map(|v| v.to_vec()).collect();

        Self {
            size: (5, 5),
            card,
            matched,
        }
    }
}

pub struct Problem4 {}

impl Problem for Problem4 {
    type Output = u64;

    fn solve_part1(&self, mut lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let draw_str = lines.next().unwrap().unwrap();
        let mut draws = draw_str.split(',').map(|n| n.parse::<u64>().unwrap());

        // empty line + 5 lines of board
        let mut boards: Vec<BingoBoard> = lines
            .map(|l| l.unwrap())
            .collect::<Vec<_>>()
            .iter()
            .chunks(6)
            .into_iter()
            .map(|chunk| From::from(chunk.collect_vec().into_iter()))
            .collect();

        let draw = draws
            .find(|draw| boards.iter_mut().map(|b| b.mark(*draw)).any(|b| b))
            .expect("a winner sometime");

        let winner = boards.iter().find(|b| b.check_victory()).unwrap();

        (draw as u64) * winner.unmatched_sum()
    }

    fn solve_part2(&self, mut lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let draw_str = lines.next().unwrap().unwrap();
        let draws = draw_str.split(',').map(|n| n.parse::<u64>().unwrap());

        // empty line + 5 lines of board
        let mut boards: Vec<BingoBoard> = lines
            .map(|l| l.unwrap())
            .collect::<Vec<_>>()
            .iter()
            .chunks(6)
            .into_iter()
            .map(|chunk| From::from(chunk.collect_vec().into_iter()))
            .collect();

        // TODO: could we turn this into some type of iterator somehow?
        // that uses a sequence and calls a function on the elements of the iterator
        // who also provide a predicate so that when T.doit(next_sequence_element)
        // returns true then our Iterator::next finally returns a value, a Vec<T> of all
        // Ts that returned true at this sequence
        for draw in draws {
            let (winners, losers): (Vec<BingoBoard>, Vec<BingoBoard>) = boards
                .into_iter()
                // TODO: weak that i had to split thi sout
                .map(|mut b| {
                    b.mark(draw);
                    b
                })
                .partition(|b| b.check_victory());

            boards = losers;

            if boards.is_empty() {
                return (draw as u64) * winners[0].unmatched_sum();
            }
        }

        panic!("could not find a winner");
    }

    fn sample_answer_part1(&self) -> Self::Output {
        4512
    }

    fn sample_answer_part2(&self) -> Self::Output {
        1924
    }
}
