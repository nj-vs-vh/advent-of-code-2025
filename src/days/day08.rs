use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

pub struct Playground;

impl Solution for Playground {
    type InputT = Vec<[i64; 3]>;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<i64>>()
                    .try_into()
                    .unwrap()
            })
            .collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut connections: Vec<(usize, usize, f64)> = Vec::new();
        for (i, a) in input.iter().enumerate() {
            for j in (i + 1)..input.len() {
                let b = input[j];
                let distance =
                    (((a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2)) as f64)
                        .sqrt();
                connections.push((i, j, distance))
            }
        }
        connections.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        let mut conn_list: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (i, j, _) in connections.iter().take(1000) {
            println!("connecting {} - {}", i, j);
            if !conn_list.contains_key(i) {
                conn_list.insert(*i, HashSet::new());
            }
            conn_list.get_mut(i).unwrap().insert(*j);

            if !conn_list.contains_key(j) {
                conn_list.insert(*j, HashSet::new());
            }
            conn_list.get_mut(j).unwrap().insert(*i);
        }

        println!("{:?}", conn_list);

        let mut visited: HashSet<usize> = HashSet::new();
        let mut circuit_sizes = Vec::<usize>::new();
        for i in 0..input.len() {
            if visited.contains(&i) {
                continue;
            }

            let mut circuit: HashSet<usize> = HashSet::new();
            let mut step: HashSet<usize> = HashSet::new();
            step.insert(i);
            while !step.is_empty() {
                for i in step.iter() {
                    circuit.insert(*i);
                }

                let mut next_step: HashSet<usize> = HashSet::new();
                for j1 in step.iter() {
                    if let Some(connected) = conn_list.get(&j1) {
                        for j2 in connected.iter() {
                            if !circuit.contains(j2) {
                                next_step.insert(*j2);
                            }
                        }
                    }
                }
                step = next_step;
            }
            circuit_sizes.push(circuit.len());
            visited.extend(circuit);
        }

        circuit_sizes.sort_by(|a, b| b.cmp(a));
        println!("{:?}", circuit_sizes);

        circuit_sizes.into_iter().take(3).reduce(|acc, v| acc * v).unwrap()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        0
    }
}
