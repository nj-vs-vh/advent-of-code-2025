use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

pub struct GiftShop;

impl Solution for GiftShop {
    type InputT = Vec<(usize, usize)>;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .take(2)
                    .map(|r| r.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut invalid_id_sum = 0;
        for id_range in input.iter() {
            for id in id_range.0..=id_range.1 {
                let idstr = id.to_string();
                if idstr.len() % 2 != 0 {
                    continue;
                }
                let mid = idstr.len() / 2;
                if idstr[..mid] == idstr[mid..] {
                    invalid_id_sum += id;
                }
            }
        }
        invalid_id_sum
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut invalid_id_sum = 0;
        for id_range in input.iter() {
            for id in id_range.0..=id_range.1 {
                let idstr = id.to_string();
                for part_count in 2..=idstr.len() {
                    if idstr.len() % part_count != 0 {
                        continue;
                    }
                    let partlen = idstr.len() / part_count;
                    let mut is_valid = false; // to check!
                    for part_idx in 1..part_count {
                        if idstr[(part_idx * partlen)..((part_idx + 1) * partlen)]
                            != idstr[..partlen]
                        {
                            is_valid = true;
                            break;
                        }
                    }
                    if !is_valid {
                        invalid_id_sum += id;
                        break;
                    }
                }
            }
        }
        invalid_id_sum
    }
}
