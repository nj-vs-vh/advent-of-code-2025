use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

type ingr = usize;

#[derive(Debug)]
pub struct IngridientDB {
    pub fresh_ranges: Vec<(ingr, ingr)>,
    pub ingridients: Vec<ingr>,
}

pub struct Cafeteria;

impl Solution for Cafeteria {
    type InputT = IngridientDB;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let (fresh_ranges_p, ingridients_p) =
            input_raw.split("\n\n").take(2).collect_tuple().unwrap();
        IngridientDB {
            fresh_ranges: fresh_ranges_p
                .lines()
                .map(|line| {
                    line.split('-')
                        .take(2)
                        .map(|i| i.parse::<ingr>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect(),
            ingridients: ingridients_p
                .lines()
                .map(|i| i.parse::<ingr>().unwrap())
                .collect(),
        }
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .ingridients
            .iter()
            .map(|i| {
                input
                    .fresh_ranges
                    .iter()
                    .any(|&range| *i >= range.0 && *i <= range.1) as usize
            })
            .sum()
    }

    fn solve_pt2(
        &self,
        mut input: Self::InputT,
        _visualizer: &mut dyn Visualizer,
    ) -> Self::OutputT {
        input.fresh_ranges.sort_by_key(|r| r.0);
        let mut res: usize = 0;
        for (this, next) in input.fresh_ranges.iter().tuple_windows() {
            res += std::cmp::min(next.0, this.1 + 1) - this.0;
        }
        res
    }
}
