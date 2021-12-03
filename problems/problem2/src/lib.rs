use framework::{read_lines, Problem};

struct Problem2 {}
impl Problem for Problem2 {
    type Output = u64;

    fn solve(&self, filepath: &str) -> Self::Output {
        let (x, y) = read_lines(filepath).unwrap().fold((0, 0), |acc, line| {
            let s = line
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let (direction, distance) = (&s[0], s[1].parse::<u64>().unwrap());
            match direction.as_str() {
                "forward" => (acc.0 + distance, acc.1),
                "down" => (acc.0, acc.1 + distance),
                "up" => (acc.0, acc.1 - distance),
                _ => panic!("Unsupported direction"),
            }
        });
        x * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem2_part1_sample() {
        let problem = Problem2 {};
        assert_eq!(
            problem.solve(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/sample.2")),
            150
        )
    }

    #[test]
    fn problem2_part1_input() {
        let problem = Problem2 {};
        assert_eq!(
            problem.solve(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/input.2")),
            2147104
        )
    }
}
