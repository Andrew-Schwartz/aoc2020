// aoc_runner_derive::aoc_main! { lib = aoc2020 }

use std::fs;
use std::time::Instant;

mod utils;
mod day01;
mod day02;

macro_rules! solve {
    ($mod:tt, $fname:literal, $day:literal) => {
        {
            use $mod::*;

            let input = fs::read_to_string(concat!("input/2020/", $fname, ".txt")).unwrap();

            let tstart = Instant::now();
            let input = gen(&input);
            let tgen = Instant::now();
            let part1 = part1(&input);
            let tpart1 = Instant::now();
            let part2 = part2(&input);
            let tpart2 = Instant::now();

            println!($day);
            println!("\tgenerator: {:?}", tgen.duration_since(tstart));
            println!("Part 1: {}", part1);
            println!("\trunner: {:?}", tpart1.duration_since(tgen));
            println!("Part 2: {}", part2);
            println!("\trunner: {:?}", tpart2.duration_since(tpart1));
            println!();
        }
    };
}

// todo add benches then make day2 unchecked :)
fn main() {
    println!("Advent of Code 2020\n");

    solve!(day01, "day1", "Day 1");

    solve!(day02, "day2", "Day 2");
}