use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;

use crate::framework::Problem;
use itertools::Itertools;
use once_cell::sync::Lazy;

pub struct Problem14 {}

impl Problem for Problem14 {
    type Output = u64;

    fn solve_part1(&self, mut lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let mut chain = lines.next().unwrap().unwrap();
        lines.next(); //trash newline
        let insert_map: HashMap<_, _> = lines
            .map(|s| s.unwrap())
            .map(|s| {
                let mut i = s.split(" -> ");
                (i.next().unwrap().to_string(), i.next().unwrap().to_string())
            })
            .collect();

        for _ in 0..10 {
            println!("{}", &chain);
            let mut newchain = chain
                .chars()
                .zip(chain.chars().skip(1))
                .map(|(c1, c2)| {
                    let s = format!("{}{}", c1, c2);
                    format!("{}{}", c1, insert_map[&s])
                })
                .join("");
            newchain.push(chain.chars().last().unwrap());
            chain = newchain;
        }

        let counts: HashMap<char, i32> = chain.chars().fold(HashMap::new(), |mut acc, n| {
            let entry = acc.entry(n).or_default();
            *entry += 1;
            acc
        });
        (counts.values().max().unwrap() - counts.values().min().unwrap()) as u64
    }

    fn solve_part2(&self, mut lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let chain = lines.next().unwrap().unwrap();
        lines.next(); //trash newline
        let insert_map: HashMap<_, (char, String, String)> = lines
            .map(|s| s.unwrap())
            .map(|s| {
                let mut i = s.split(" -> ");
                let key = i.next().unwrap().to_string();
                let value = i.next().unwrap().to_string();
                let mut key_iter = key.chars();
                let pair1 = format!("{}{}", key_iter.next().unwrap(), value);
                let pair2 = format!("{}{}", value, key_iter.next().unwrap());
                (key, (value.chars().next().unwrap(), pair1, pair2))
            })
            .collect();

        let mut count_map: HashMap<String, i64> = HashMap::new();
        chain
            .chars()
            .zip(chain.chars().skip(1))
            .for_each(|(c1, c2)| {
                let s = format!("{}{}", c1, c2);
                *count_map.entry(s).or_insert(0) += 1;
            });

        let mut char_map: HashMap<char, i64> = HashMap::new();
        for c in chain.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }
        dbg!(&char_map, &count_map);

        for _ in 0..40 {
            /*println!(
                "{}",
                count_map
                    .keys()
                    .sorted()
                    .map(|k| format!("({}, {})", k, count_map[k]))
                    .join(", ")
            );*/

            let mut new_map = HashMap::new();
            count_map.iter().for_each(|(k, v)| {
                let (c, p1, p2) = insert_map.get(k).unwrap();

                *new_map.entry(p1.clone()).or_insert(0) += v;
                *new_map.entry(p2.clone()).or_insert(0) += v;
                *char_map.entry(*c).or_insert(0) += v;
            });
            count_map = new_map;
        }

        dbg!(&char_map);
        dbg!(&count_map);
        (char_map.values().max().unwrap() - char_map.values().min().unwrap()) as u64
    }

    fn sample_answer_part1(&self) -> Self::Output {
        1588
    }

    // another with no answer for part2
    fn sample_answer_part2(&self) -> Self::Output {
        2188189693529
    }
}
