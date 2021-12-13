use itertools::Itertools;
use once_cell::sync::Lazy;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::rc::Rc;

use crate::framework::Problem;
use crate::util::has_unique_elements;

pub struct Node {
    id: String,
    nodes: RefCell<Vec<Rc<Node>>>,
}

pub struct Graph {
    nodes: HashMap<String, Rc<Node>>,
}

impl Graph {
    fn find_all_paths_with(&self, f: fn(&[String], &String) -> bool) -> Vec<Vec<String>> {
        fn inner(
            nodes: &HashMap<String, Rc<Node>>,
            paths: Vec<Vec<String>>,
            f: fn(&[String], &String) -> bool,
        ) -> Vec<Vec<String>> {
            // TODO: how do i do this without this dumb marker
            let mut more = false;
            let paths = paths
                .into_iter()
                .flat_map::<Vec<Vec<String>>, _>(|path| {
                    let node = nodes.get(path.iter().last().unwrap()).unwrap();

                    if node.id == "end" {
                        // TODO: hmmm, this is also dirty
                        vec![path]
                    } else {
                        node.nodes
                            .borrow()
                            .iter()
                            .filter_map(|n| {
                                (f(&path, &n.id)).then(|| {
                                    let mut v = path.clone();
                                    v.push(n.id.clone());
                                    more = true;
                                    v
                                })
                            })
                            .collect()
                    }
                })
                .collect();

            if more {
                inner(nodes, paths, f)
            } else {
                paths
            }
        }

        inner(&self.nodes, vec![vec!["start".to_string()]], f)
    }
}

pub struct Problem12 {}

impl Problem for Problem12 {
    type Output = u64;

    fn solve_part1(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let mut nodes = HashMap::new();
        for s in lines.map(|s| s.unwrap()) {
            let mut iter = s.split('-');
            let (n1, n2) = (iter.next().unwrap(), iter.next().unwrap());
            let node1 = nodes
                .entry(n1.to_string())
                .or_insert_with(|| {
                    Rc::new(Node {
                        id: n1.to_string(),
                        nodes: RefCell::new(vec![]),
                    })
                })
                .clone();
            let node2 = nodes
                .entry(n2.to_string())
                .or_insert_with(|| {
                    Rc::new(Node {
                        id: n2.to_string(),
                        nodes: RefCell::new(vec![]),
                    })
                })
                .clone();
            node1.nodes.borrow_mut().push(node2.clone());
            node2.nodes.borrow_mut().push(node1.clone());
        }
        let graph = Graph { nodes };

        graph
            .find_all_paths_with(|path, id| (id.to_lowercase() != *id || !path.contains(id)))
            .len() as u64
    }

    fn solve_part2(&self, lines: io::Lines<io::BufReader<File>>) -> Self::Output {
        let mut nodes = HashMap::new();
        for s in lines.map(|s| s.unwrap()) {
            let mut iter = s.split('-');
            let (n1, n2) = (iter.next().unwrap(), iter.next().unwrap());
            let node1 = nodes
                .entry(n1.to_string())
                .or_insert_with(|| {
                    Rc::new(Node {
                        id: n1.to_string(),
                        nodes: RefCell::new(vec![]),
                    })
                })
                .clone();
            let node2 = nodes
                .entry(n2.to_string())
                .or_insert_with(|| {
                    Rc::new(Node {
                        id: n2.to_string(),
                        nodes: RefCell::new(vec![]),
                    })
                })
                .clone();
            node1.nodes.borrow_mut().push(node2.clone());
            node2.nodes.borrow_mut().push(node1.clone());
        }
        let graph = Graph { nodes };

        let paths = graph.find_all_paths_with(|path, id| {
            id.to_lowercase() != *id
                || !path.contains(id)
                || (has_unique_elements(path.iter().filter(|p| **p == p.to_lowercase()))
                    && id != "start")
        });
        paths.len() as u64
    }

    fn sample_answer_part1(&self) -> Self::Output {
        226
    }

    fn sample_answer_part2(&self) -> Self::Output {
        3509
    }
}
