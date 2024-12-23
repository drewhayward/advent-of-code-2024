use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::solution::Solution;

type Node = [char; 2];
type Edge = (Node, Node);
type EdgeList = HashMap<Node, HashSet<Node>>;

fn build_edge_list(edges: &[Edge]) -> EdgeList {
    let mut edge_list = HashMap::new();
    for (a, b) in edges.iter() {
        let a_list = edge_list.entry(a.clone()).or_insert(HashSet::new());
        a_list.insert(b.clone());

        let b_list = edge_list.entry(b.clone()).or_insert(HashSet::new());
        b_list.insert(a.clone());
    }

    edge_list
}

fn find_3_cliques(edges: &[Edge], edge_list: &EdgeList) -> HashSet<(Node, Node, Node)> {
    let mut cliques = HashSet::new();
    for (a, b) in edges {
        if let (Some(alist), Some(blist)) = (edge_list.get(a), edge_list.get(b)) {
            for node in alist.intersection(blist) {
                let mut clique = vec![a.clone(), b.clone(), *node];
                clique.sort();
                cliques.insert((clique[0].clone(), clique[1].clone(), clique[2].clone()));
            }
        }
    }

    cliques
}

fn bron_kerbosch(
    edge_list: &EdgeList,
    R: HashSet<Node>,
    P: HashSet<Node>,
    X: HashSet<Node>,
    cliques: &mut Vec<HashSet<Node>>,
) {
    if P.is_empty() && X.is_empty() {
        cliques.push(R.clone());
    }

    if P.is_empty() {
        return;
    }

    let pivot = P.union(&X).next().unwrap();
    let vertices = P.difference(edge_list.get(pivot).unwrap());
    for v in vertices {
        let mut sub_r = R.clone();
        sub_r.insert(*v);
        bron_kerbosch(
            edge_list,
            sub_r,
            P.intersection(edge_list.get(v).unwrap()).cloned().collect(),
            X.intersection(edge_list.get(v).unwrap()).cloned().collect(),
            cliques,
        );
    }
}

pub struct LanParty;

impl Solution for LanParty {
    fn part1(puzzle_input: String) -> String {
        let edges: Vec<Edge> = puzzle_input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("-").unwrap();
                (
                    [a.chars().nth(0).unwrap(), a.chars().nth(1).unwrap()],
                    [b.chars().nth(0).unwrap(), b.chars().nth(1).unwrap()],
                )
            })
            .collect();
        let edge_list = build_edge_list(&edges);
        let cliques = find_3_cliques(&edges, &edge_list);

        cliques
            .iter()
            .filter(|(a, b, c)| a[0] == 't' || b[0] == 't' || c[0] == 't')
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let edges: Vec<Edge> = puzzle_input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("-").unwrap();
                (
                    [a.chars().nth(0).unwrap(), a.chars().nth(1).unwrap()],
                    [b.chars().nth(0).unwrap(), b.chars().nth(1).unwrap()],
                )
            })
            .collect();
        let edge_list = build_edge_list(&edges);
        let mut cliques = Vec::new();

        bron_kerbosch(
            &edge_list,
            HashSet::new(),
            edge_list.keys().cloned().collect(),
            HashSet::new(),
            &mut cliques,
        );

        let mut largest_clique: Vec<Node> = cliques
            .iter()
            .max_by_key(|c| c.len())
            .unwrap()
            .iter()
            .cloned()
            .collect();
        largest_clique.sort();

        largest_clique
            .iter()
            .map(|c| format!("{}{}", c.get(0).unwrap(), c.get(1).unwrap()))
            .join(",")
    }
}
