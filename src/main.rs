#![allow(dead_code)]

mod color;
mod days;
mod solution;
mod text_to_image;
mod types;
mod utils;
mod visualizer;

use std::path::PathBuf;

use clap::Parser;
use utils::read_input;

use crate::{
    solution::Solution,
    visualizer::{DisabledVisualizer, GifVisualizer, TerminalVisualizer, Visualizer},
};

#[derive(Parser, Debug)]
#[command(name = "AoC 2022 solutions")]
#[command(author = "Igor V. <gosha.vaiman@gmail.com>")]
#[command(version = "1.3.1.2")]
struct CliArgs {
    day: u8,

    #[arg(short, long, default_value_t = false)]
    example: bool,

    #[arg(value_enum, default_value_t = types::RunPart::Both)]
    part: types::RunPart,

    #[arg(short, long, default_value_t = false)]
    visualize: bool,

    #[arg(long, default_value_t = 30.0)]
    fps: f32,

    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    #[arg(long, value_name = "FILE")]
    gif: Option<PathBuf>,

    #[arg(long, default_value_t = 800)]
    gif_width: u32,
}

fn main() {
    let args = CliArgs::parse();
    println!("AoC 2025, day {}", args.day);

    let read_input_result = read_input(args.day, args.example);
    if let Err(e) = read_input_result {
        println!("Error reading input file ({})!", e);
        return;
    }
    let input = read_input_result.unwrap();
    let part = args.part;

    let vis: Box<dyn Visualizer> = match args.visualize {
        true => {
            if let Some(gif_path) = args.gif {
                Box::new(GifVisualizer::new(
                    gif_path.to_str().unwrap(),
                    args.fps,
                    args.gif_width,
                ))
            } else {
                Box::new(TerminalVisualizer::new(args.fps, args.interactive))
            }
        }
        false => Box::new(DisabledVisualizer {}),
    };

    match args.day {
        1 => days::day01::SecretEntrance.run(input, part, vis),
        2 => days::day02::GiftShop.run(input, part, vis),
        3 => days::day03::Lobby.run(input, part, vis),
        4 => days::day04::PrintingDepartment.run(input, part, vis),
        5 => days::day05::Cafeteria.run(input, part, vis),
        6 => days::day06::TrashCompactor.run(input, part, vis),
        7 => days::day07::Laboratories.run(input, part, vis),
        8 => days::day08::Playground.run(input, part, vis),
        _ => {
            println!("Solution is not yet implemented");
        }
    }
}
