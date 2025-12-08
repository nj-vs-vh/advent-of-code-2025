use std::collections::{HashMap, HashSet};

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
        let connections = make_connections(&input);

        let mut conn_list: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (i, j, _) in connections.iter().take(1000) {
            if !conn_list.contains_key(i) {
                conn_list.insert(*i, HashSet::new());
            }
            conn_list.get_mut(i).unwrap().insert(*j);

            if !conn_list.contains_key(j) {
                conn_list.insert(*j, HashSet::new());
            }
            conn_list.get_mut(j).unwrap().insert(*i);
        }

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

        circuit_sizes
            .into_iter()
            .take(3)
            .reduce(|acc, v| acc * v)
            .unwrap()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut circuit_members: HashMap<usize, HashSet<usize>> = (0..input.len())
            .map(|jid| {
                let mut s = HashSet::new();
                s.insert(jid);
                (jid, s)
            })
            .collect();
        let mut next_circuit_id: usize = input.len();
        let mut circuit: HashMap<usize, usize> = (0..input.len()).map(|jid| (jid, jid)).collect();
        for (i, j, _) in make_connections(&input) {
            let circuit_id_i = circuit.get(&i).unwrap();
            let circuit_id_j = circuit.get(&j).unwrap();
            if circuit_id_i == circuit_id_j {
                continue; // no effect on the circuits
            }

            let mut new_circuit_members = HashSet::<usize>::new();
            new_circuit_members.extend(circuit_members.remove(circuit_id_i).unwrap());
            new_circuit_members.extend(circuit_members.remove(circuit_id_j).unwrap());

            for id in new_circuit_members.iter() {
                circuit.insert(*id, next_circuit_id); // reassign all members to the newly created joint circuit
            }
            circuit_members.insert(next_circuit_id, new_circuit_members);
            next_circuit_id += 1;

            if next_circuit_id > 2 && circuit_members.len() == 1 {
                println!("{:?}", input[i]);
                println!("{:?}", input[j]);
                return (input[i][0] * input[j][0]) as usize;
            }
        }
        0
    }
}

fn make_connections(input: &Vec<[i64; 3]>) -> Vec<(usize, usize, f64)> {
    let mut connections: Vec<(usize, usize, f64)> = Vec::new();
    for (i, a) in input.iter().enumerate() {
        for j in (i + 1)..input.len() {
            let b = input[j];
            let distance = (((a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2))
                as f64)
                .sqrt();
            connections.push((i, j, distance))
        }
    }
    connections.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    connections
}
