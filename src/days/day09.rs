use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

pub struct MovieTheater;

impl Solution for MovieTheater {
    type InputT = Vec<(u64, u64)>;
    type OutputT = u64;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        (0..(input.len() - 1))
            .map(|i| ((i + 1)..input.len()).map(move |j| (i, j)))
            .flatten()
            .map(|(i, j)| area(&input[i], &input[j]))
            .max()
            .unwrap()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut areas = (0..(input.len() - 1))
            .map(|i| ((i + 1)..input.len()).map(move |j| (i, j)))
            .flatten()
            .map(|(i, j)| area(&input[i], &input[j]))
            .collect::<Vec<_>>();
        areas.sort();
        0
    }
}

fn area(a: &(u64, u64), b: &(u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

// fn is_inscribed(a: &(u64, u64), b: &(u64, u64), outline: &Vec<(u64, u64)>) -> bool {
//     false
// }
