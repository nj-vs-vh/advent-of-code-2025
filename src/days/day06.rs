use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

#[derive(Debug)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug)]
struct Problem {
    operands_1: Vec<u64>,
    operands_2: Vec<u64>,
    op: Op,
}

#[derive(Debug)]
pub struct Homework {
    problems: Vec<Problem>,
}

pub struct TrashCompactor;

impl Solution for TrashCompactor {
    type InputT = Homework;
    type OutputT = u64;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let lines: Vec<&str> = input_raw.lines().collect();
        let operand_lines: Vec<Vec<char>> = lines[..lines.len() - 1]
            .iter()
            .map(|line| line.chars().collect())
            .collect();
        let op_line: Vec<char> = lines[lines.len() - 1].chars().collect();

        let mut block_bounds: Vec<usize> = lines[lines.len() - 1]
            .chars()
            .enumerate()
            .filter_map(|(idx, ch)| if ch != ' ' { Some(idx) } else { None })
            .collect();
        block_bounds.push(op_line.len() + 1);

        let problems = block_bounds
            .iter()
            .tuple_windows()
            .map(|(&col_st, &col_end)| Problem {
                op: match op_line[col_st] {
                    '+' => Op::Add,
                    '*' => Op::Mult,
                    err => panic!("Unexpected op sign: {}", err),
                },
                operands_1: operand_lines
                    .iter()
                    .map(|line| {
                        line[col_st..col_end - 1]
                            .iter()
                            .collect::<String>()
                            .trim()
                            .parse::<u64>()
                            .unwrap()
                    })
                    .collect(),
                operands_2: (col_st..col_end - 1)
                    .rev()
                    .map(|col| {
                        operand_lines
                            .iter()
                            .map(|line| line[col])
                            .collect::<String>()
                            .trim()
                            .parse::<u64>()
                            .unwrap()
                    })
                    .collect(),
            });

        Homework {
            problems: problems.collect(),
        }
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .problems
            .into_iter()
            .map(|problem| {
                problem
                    .operands_1
                    .into_iter()
                    .reduce(|acc, el| match problem.op {
                        Op::Add => acc + el,
                        Op::Mult => acc * el,
                    })
                    .unwrap()
            })
            .sum()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .problems
            .into_iter()
            .map(|problem| {
                problem
                    .operands_2
                    .into_iter()
                    .reduce(|acc, el| match problem.op {
                        Op::Add => acc + el,
                        Op::Mult => acc * el,
                    })
                    .unwrap()
            })
            .sum()
    }
}
