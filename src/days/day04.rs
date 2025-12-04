use itertools::Itertools;

use crate::{solution::Solution, utils::manhattan_neighborhood, visualizer::Visualizer};

pub struct PrintingDepartment;

impl Solution for PrintingDepartment {
    type InputT = Vec<Vec<bool>>;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .lines()
            .map(|line| line.chars().map(|ch| ch == '@').collect())
            .collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut res: usize = 0;

        let height = input.len();
        let width = input[0].len();
        for i in 0..height {
            for j in 0..width {
                if !input[i][j] {
                    continue;
                }
                let neighbors: u8 = manhattan_neighborhood(&i, &j, &width, &height)
                    .iter()
                    .map(|(i_n, j_n)| input[*i_n][*j_n] as u8)
                    .sum();
                if neighbors < 4 {
                    res += 1;
                }
            }
        }
        res
    }

    fn solve_pt2(
        &self,
        mut input: Self::InputT,
        _visualizer: &mut dyn Visualizer,
    ) -> Self::OutputT {
        let rolls_start: usize = input
            .iter()
            .flatten()
            .map(|&is_roll| is_roll as usize)
            .sum();

        let height = input.len();
        let width = input[0].len();
        let mut neighbors: Vec<Vec<u8>> = vec![vec![0; width]; height];
        for i in 0..height {
            for j in 0..width {
                if !input[i][j] {
                    continue;
                }
                neighbors[i][j] = manhattan_neighborhood(&i, &j, &width, &height)
                    .iter()
                    .map(|(i_n, j_n)| input[*i_n][*j_n] as u8)
                    .sum();
            }
        }

        loop {
            let accessible: Vec<(usize, usize)> = (0..height)
                .cartesian_product(0..width)
                .filter(|(i, j)| input[*i][*j] && neighbors[*i][*j] < 4)
                .collect();
            if accessible.len() == 0 {
                break;
            }

            for (i, j) in accessible.into_iter() {
                input[i][j] = false;
                for (i_n, j_n) in manhattan_neighborhood(&i, &j, &width, &height) {
                    if input[i_n][j_n] {
                        neighbors[i_n][j_n] -= 1;
                    }
                }
            }
        }

        let rolls_end: usize = input
            .iter()
            .flatten()
            .map(|&is_roll| is_roll as usize)
            .sum();

        rolls_start - rolls_end
    }
}
