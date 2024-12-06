use std::collections::HashMap;

use crate::solution::Solution;

type Edge = (u32, u32);

#[derive(Debug)]
struct Graph {
    forward_edges: HashMap<u32, Vec<u32>>,
    backward_edges: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn build(nodes: Vec<u32>, edges: &[Edge]) -> Graph {
        let mut forward_edges = HashMap::new();
        let mut backward_edges = HashMap::new();

        for node in nodes {
            forward_edges.entry(node).or_insert(Vec::new());
            backward_edges.entry(node).or_insert(Vec::new());
        }

        for edge in edges {
            forward_edges
                .entry(edge.0)
                .or_insert(Vec::new())
                .push(edge.1);
            backward_edges
                .entry(edge.1)
                .or_insert(Vec::new())
                .push(edge.0);
        }

        Graph {
            forward_edges,
            backward_edges,
        }
    }

    fn roots(&self) -> Vec<u32> {
        self.backward_edges
            .iter()
            .filter_map(|(k, v)| v.is_empty().then_some(k))
            .copied()
            .collect()
    }

    fn afters(&self, num: u32) -> Option<Vec<u32>> {
        self.forward_edges.get(&num).cloned()
    }

    fn remove_edge(&mut self, edge: Edge) {
        // Remove forward edge
        let f = self.forward_edges.get_mut(&edge.0).unwrap();
        let f_index = f.iter().position(|&n| n == edge.1).unwrap();
        f.remove(f_index);

        // Remove backward edge
        let b = self.backward_edges.get_mut(&edge.1).unwrap();
        let b_index = b.iter().position(|&n| n == edge.0).unwrap();
        b.remove(b_index);
    }
}

fn build_num_to_index(list: &[u32]) -> HashMap<u32, usize> {
    list.iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect()
}

pub struct PrintOrder;

impl PrintOrder {
    fn parse_input(puzzle_input: String) -> (Vec<Edge>, Vec<Vec<u32>>) {
        let mut parts = puzzle_input.split("\n\n");

        let constraints = parts
            .next()
            .expect("Should have a constraints part")
            .lines()
            .map(|l| l.split_once('|').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect();

        let lists: Vec<Vec<_>> = parts
            .next()
            .expect("Should have a lists part")
            .lines()
            .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
            .collect();

        (constraints, lists)
    }

    fn is_in_order(list: &[u32], constraints: &[Edge]) -> bool {
        let num_to_index: HashMap<u32, usize> = build_num_to_index(list);

        for constr in constraints {
            match (num_to_index.get(&constr.0), num_to_index.get(&constr.1)) {
                (Some(b), Some(a)) if b >= a => return false,
                _ => continue,
            }
        }

        true
    }

    /// Use a topological sort to build the correct ordering of the numbers
    fn fix_order(list: &[u32], constraints: &[Edge]) -> Vec<u32> {
        let edges: Vec<_> = constraints
            .iter()
            .filter(|c| list.contains(&c.0) && list.contains(&c.1))
            .copied()
            .collect();

        let mut output = Vec::new();
        let mut graph = Graph::build(list.to_vec(), &edges);
        let mut roots = graph.roots();

        while !roots.is_empty() {
            let node = roots.pop().unwrap();
            output.push(node);
            for m in graph.afters(node).unwrap() {
                graph.remove_edge((node, m));
                if graph.backward_edges.get(&m).unwrap().is_empty() {
                    roots.push(m);
                }
            }
        }

        return output;
    }
}

impl Solution for PrintOrder {
    fn part1(puzzle_input: String) -> String {
        let (constraints, lists) = PrintOrder::parse_input(puzzle_input);

        lists
            .iter()
            .filter(|l| PrintOrder::is_in_order(l, &constraints))
            .map(|l| l.get(l.len() / 2).unwrap())
            .sum::<u32>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (constraints, mut lists) = PrintOrder::parse_input(puzzle_input);

        lists
            .iter_mut()
            .filter(|l| !PrintOrder::is_in_order(l, &constraints))
            .map(|l| PrintOrder::fix_order(l, &constraints))
            .map(|l| l.get(l.len() / 2).unwrap().clone())
            .sum::<u32>()
            .to_string()
    }
}
