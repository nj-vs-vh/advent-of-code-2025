use crate::{solution::Solution, visualizer::Visualizer};

pub struct DayName;

impl Solution for DayName {
    type InputT = usize;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        0
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        0
    }

    fn solve_pt2(&self, input: Self::InputT, visualizer: &mut dyn Visualizer) -> Self::OutputT {
        0
    }
}
