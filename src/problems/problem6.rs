use itertools::Itertools;

use crate::framework::Problem;

const TURN_SIZE: usize = 9;

struct Reproducer {
    fish: [u64; TURN_SIZE],
}

impl From<&str> for Reproducer {
    fn from(s: &str) -> Self {
        let turns = s.split(',').map(|v| v.parse::<usize>().unwrap());
        let fish = turns.fold([0u64; TURN_SIZE], |mut acc, n| {
            acc[n] += 1;
            acc
        });
        Self { fish }
    }
}

impl Reproducer {
    fn step(&mut self) {
        let repro = self.fish[0];
        for i in 1..TURN_SIZE {
            self.fish[i - 1] = self.fish[i];
        }
        // reproduction adds to 6 and 8 turns
        self.fish[6] += repro;
        // Note: set the 8th time
        self.fish[8] = repro;
    }

    fn count(&self) -> u64 {
        self.fish.iter().sum()
    }
}

impl std::fmt::Display for Reproducer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.fish.iter().map(|v| v.to_string()).join(", "))
    }
}

pub struct Problem6 {}

impl Problem for Problem6 {
    type Output = u64;

    fn solve_part1(
        &self,
        mut lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> <Self as Problem>::Output {
        let fish = lines.next().unwrap().unwrap();
        let mut repro = Reproducer::from(fish.as_str());

        for _ in 0..80 {
            repro.step();
        }
        repro.count()
    }

    fn solve_part2(
        &self,
        mut lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> <Self as Problem>::Output {
        let fish = lines.next().unwrap().unwrap();
        let mut repro = Reproducer::from(fish.as_str());

        for _ in 0..256 {
            repro.step();
        }
        repro.count()
    }

    fn sample_answer_part1(&self) -> <Self as Problem>::Output {
        5934
    }

    fn sample_answer_part2(&self) -> <Self as Problem>::Output {
        26984457539
    }
}
