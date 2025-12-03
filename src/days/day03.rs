use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

pub struct Lobby;

impl Solution for Lobby {
    type InputT = Vec<Vec<u8>>;
    type OutputT = u64;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .lines()
            .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect())
            .collect()
    }
    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input.into_iter().map(|bank| max_joltage(bank, 2)).sum()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input.into_iter().map(|bank| max_joltage(bank, 12)).sum()
    }
}

fn max_joltage(mut bank: Vec<u8>, nbatteries: usize) -> u64 {
    bank.reverse();
    let mut res: u64 = 0;
    let mut stop = bank.len();
    for ibatt in 0..nbatteries {
        let start = nbatteries - ibatt - 1;
        let idx = bank[start..stop].iter().position_max().unwrap() + start;
        stop = idx;

        res += (bank[idx] as u64) * 10u64.pow(start as u32);
    }
    res
}
