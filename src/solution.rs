use std::{fmt::Display, time::Instant};

use crate::types::RunPart;
use crate::utils::ascii_box;
use crate::visualizer::Visualizer;

pub trait Solution {
    type InputT;
    type OutputT: Display;

    fn run(&self, input_raw: String, part: RunPart, mut visualizer: Box<dyn Visualizer>) {
        if part == RunPart::Pt1 || part == RunPart::Both {
            let input_raw_clone = input_raw.clone();
            let start = Instant::now();
            let input = self.parse_input(input_raw_clone);
            let output = self.solve_pt1(input, visualizer.as_mut());
            println!(
                "\nPart 1 solution (took {:.3} msec):\n{}",
                start.elapsed().as_secs_f32() * 1000.0,
                ascii_box(format!("{}", output), 1, 35)
            );
        }
        if part == RunPart::Pt2 || part == RunPart::Both {
            let start = Instant::now();
            let input = self.parse_input(input_raw);
            let output = self.solve_pt2(input, visualizer.as_mut());
            println!(
                "\nPart 2 solution (took {:.3} msec):\n{}",
                start.elapsed().as_secs_f32() * 1000.0,
                ascii_box(format!("{}", output), 1, 35)
            );
        }
    }

    fn parse_input(&self, input_raw: String) -> Self::InputT;

    fn solve_pt1(&self, input: Self::InputT, visualizer: &mut dyn Visualizer) -> Self::OutputT;

    fn solve_pt2(&self, input: Self::InputT, visualizer: &mut dyn Visualizer) -> Self::OutputT;
}
