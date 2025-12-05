use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

type Ingr = usize;

#[derive(Debug)]
pub struct IngridientDB {
    pub fresh_ranges: Vec<(Ingr, Ingr)>,
    pub ingridients: Vec<Ingr>,
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
                    let (start, end) = line
                        .split('-')
                        .take(2)
                        .map(|i| i.parse::<Ingr>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (start, end + 1) // storing upper end as exclusive bound
                })
                .collect(),
            ingridients: ingridients_p
                .lines()
                .map(|i| i.parse::<Ingr>().unwrap())
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
                    .any(|&range| *i >= range.0 && *i < range.1) as usize
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
        for (idx, range) in input.fresh_ranges.iter().enumerate() {
            if idx == input.fresh_ranges.len() - 1 {
                res += range.1 - range.0; // last range
            }

            let mut right = *range;
            for to_subtract in input.fresh_ranges[(idx + 1)..].iter() {
                let left = (right.0, std::cmp::min(right.1, to_subtract.0));
                if left.1 < left.0 {
                    break;
                }
                res += left.1 - left.0;

                right = (to_subtract.1, right.1);
                if right.1 <= right.0 {
                    break;
                }
            }
        }
        res
    }
}
