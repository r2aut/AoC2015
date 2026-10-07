use anyhow::Result;
use aoc2015::{AOC, AOCPrint, P1, P2};
use clap::Parser;
use std::{fs::File, io::BufReader, path::PathBuf};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    #[arg(short, long, global = true, value_name = "FILE", value_parser = clap::value_parser!(PathBuf))]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let t0 = std::time::Instant::now();
    let args = Args::parse();
    let input_file = args.input.unwrap_or_else(|| PathBuf::from(format!("puzzles/day{:02}.txt", args.day)));
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let (res_1, res_2) = match args.day {
        1 => day01::solve(reader)?,
        2 => day02::solve(reader)?,
        3 => day03::solve(reader)?,
        4 => day04::solve(reader)?,
        5 => day05::solve(reader)?,
        6 => day06::solve(reader)?,
        7 => day07::solve(reader)?,
        8 => day08::solve(reader)?,
        9 => day09::solve(reader)?,
        10 => day10::solve(reader)?,
        11 => day11::solve(reader)?,
        12 => day12::solve(reader)?,
        13 => day13::solve(reader)?,
        15 => day15::solve(reader)?,
        16 => day16::solve(reader)?,
        17 => day17::solve(reader)?,
        18 => day18::solve(reader)?,
        19 => day19::solve(reader)?,
        20 => day20::solve(reader)?,
        21 => day21::solve(reader)?,
        22 => day22::solve(reader)?,
        23 => day23::solve(reader)?,
        24 => day24::solve(reader)?,
        25 => day25::solve(reader)?,
        _ => unimplemented!("day{:02} is not implemented", args.day),
    };
    let duration = t0.elapsed();

    let day = format!("day{:02}", args.day);
    println!("{} {}", AOC, day);
    println!("-------------");

    res_1.aoc_print(P1);
    res_2.aoc_print(P2);

    println!("-------------");
    println!("Duration {} ms", duration.as_millis());

    Ok(())
}
