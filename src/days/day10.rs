use itertools::Itertools;

use crate::{solution::Solution, visualizer::Visualizer};

#[derive(Debug)]
pub struct MachineManual {
    length: usize,
    lights: u16,
    buttons: Vec<u16>,
}

impl MachineManual {
    fn parse(line: &str) -> MachineManual {
        let (mut lights_str, rest): (&str, &str) = line.split(']').collect_tuple().unwrap();
        lights_str = lights_str.strip_prefix('[').unwrap();

        let (buttons_str, _joltage_str) = rest.split('{').collect_tuple().unwrap();

        MachineManual {
            length: lights_str.chars().count(),
            lights: lights_str
                .char_indices()
                .map(|(idx, ch)| if ch == '.' { 0 } else { 1 << idx })
                .sum(),
            buttons: buttons_str
                .split(' ')
                .filter_map(|bstr| {
                    Some(
                        bstr.strip_prefix('(')?
                            .strip_suffix(')')?
                            .split(',')
                            .map(|lidx| 1u16 << lidx.parse::<usize>().unwrap())
                            .sum(),
                    )
                })
                .collect(),
        }
    }
}

impl std::fmt::Display for MachineManual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lights_prefix = "lights: ";
        let button_prefix = (0..lights_prefix.chars().count())
            .map(|_| ' ')
            .collect::<String>();

        f.write_fmt(format_args!(
            "{}{:0>length$}\nbuttons:\n",
            lights_prefix,
            format!("{:b}", self.lights),
            length = self.length,
        ))?;
        for btn in self.buttons.iter() {
            f.write_fmt(format_args!(
                "{}{:0>length$}\n",
                button_prefix,
                format!("{:b}", btn),
                length = self.length,
            ))?;
        }
        Ok(())
    }
}

pub struct Factory;

impl Solution for Factory {
    type InputT = Vec<MachineManual>;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw.lines().map(|l| MachineManual::parse(l)).collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut res = 0;
        for mm in input.iter() {
            println!("{}", mm);
            let mut button_counts: Vec<u32> = Vec::new();
            for button_mask in 0..(1usize << mm.buttons.len()) {
                // println!("{:b}", button_mask);
                let res = mm
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| (1 << idx) & button_mask > 0)
                    .fold(0, |acc, (_, button)| acc ^ button);
                if res == mm.lights {
                    button_counts.push(button_mask.count_ones());
                }
            }
            res += button_counts.iter().min().unwrap();
        }
        res
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        0
    }
}
