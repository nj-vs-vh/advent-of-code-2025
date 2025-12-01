use num::Integer;

use crate::{solution::Solution, visualizer::Visualizer};

pub struct SecretEntrance;

impl Solution for SecretEntrance {
    type InputT = Vec<i16>;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .lines()
            .map(|line| {
                (if line.chars().next().unwrap() == 'L' {
                    -1
                } else {
                    1
                }) * line[1..].parse::<i16>().unwrap()
            })
            .collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut dial: i16 = 50;
        let mut counter: usize = 0;
        for rot in input.iter() {
            dial = (dial + rot).rem_euclid(100);
            if dial == 0 {
                counter += 1;
            }
        }
        return counter;
    }

    fn solve_pt2(&self, input: Self::InputT, visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut dial: i16 = 50;
        let mut counter: usize = 0;

        for rot in input.iter() {
            visualizer.write_line(&format!("dial @ {}", dial));

            let (cycles, rot_partial) = rot.div_rem(&DIALSIZE);
            visualizer.write_line(&format!("{} -> {} cycles + {}", rot, cycles, rot_partial));
            let mut zero_crossings = cycles.abs();

            let new_dial = dial + rot_partial;
            if (dial > 0 && new_dial <= 0) || (dial < DIALSIZE && new_dial >= DIALSIZE) {
                zero_crossings += 1;
            }

            dial = new_dial.rem_euclid(100);

            counter += zero_crossings as usize;
            visualizer.write_line(&format!(
                "dial now at {}; crossed zero {} times",
                dial, zero_crossings
            ));
            visualizer.end_frame();
        }
        return counter;
    }
}

const DIALSIZE: i16 = 100;
