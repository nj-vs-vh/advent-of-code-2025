use std::collections::{HashMap, VecDeque};

use crate::{solution::Solution, visualizer::Visualizer};

#[derive(Debug)]
pub struct TachyonManifold {
    width: usize,
    height: usize,
    start: (usize, usize),
    splitters: Vec<Vec<bool>>,
}

pub struct Laboratories;

impl Solution for Laboratories {
    type InputT = TachyonManifold;
    type OutputT = u64;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let char_mat: Vec<Vec<char>> = input_raw.lines().map(|l| l.chars().collect()).collect();
        let mut start: Option<(usize, usize)> = None;
        for (i, row) in char_mat.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if *ch == 'S' {
                    start = Some((i, j))
                }
            }
        }

        TachyonManifold {
            width: char_mat[0].len(),
            height: char_mat.len(),
            start: start.unwrap(),
            splitters: char_mat
                .iter()
                .map(|row| row.iter().map(|ch| *ch == '^').collect())
                .collect(),
        }
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut is_lit: Vec<Vec<bool>> = vec![vec![false; input.width]; input.height];
        let mut ray_queue: VecDeque<(usize, usize)> = VecDeque::new();
        ray_queue.push_back(input.start);

        let mut lit_splitters = 0;

        while let Some((mut ray_i, ray_j)) = ray_queue.pop_front() {
            while ray_i < input.height - 1 && !is_lit[ray_i][ray_j] {
                is_lit[ray_i][ray_j] = true;
                if input.splitters[ray_i][ray_j] {
                    lit_splitters += 1;
                    if ray_j > 0 {
                        ray_queue.push_back((ray_i, ray_j - 1));
                    }
                    if ray_j < input.width - 1 {
                        ray_queue.push_back((ray_i, ray_j + 1));
                    }
                    break;
                }
                ray_i += 1;
            }
        }

        lit_splitters
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        count_timelines(&input, input.start, &mut HashMap::new())
    }
}

fn count_timelines(
    manifold: &TachyonManifold,
    start: (usize, usize),
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&res) = cache.get(&start) {
        return res;
    }

    let (mut i, j) = start;

    while i < manifold.height - 1 {
        if manifold.splitters[i][j] {
            let res = if j > 0 {
                count_timelines(manifold, (i, j - 1), cache)
            } else {
                0
            } + if j < manifold.width - 1 {
                count_timelines(manifold, (i, j + 1), cache)
            } else {
                0
            };
            cache.insert(start, res);
            return res;
        }
        i += 1;
    }

    1 // reached the end without splitting
}
