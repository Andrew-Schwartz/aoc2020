mod utils;
mod day01;
mod day02;

macro_rules! solve {
    (file $mod:tt, $day:literal, $fname:literal) => {
        {
            let input = std::fs::read_to_string(concat!("input/2020/", $fname, ".txt")).unwrap();
            solve!(internal $mod, $day, input)
        }
    };
    (include $mod:tt, $day:literal, $fname: literal) => {
        {
            const INPUT: &'static str = include_str!(concat!("../input/2020/", $fname, ".txt"));
            solve!(internal $mod, $day, INPUT)
        }
    };
    (internal $mod:tt, $day:literal, $input:expr) => {
        {
            use std::time::Instant;
            use $mod::*;

            println!($day);

            let tstart = Instant::now();
            let input = gen(&$input);
            let tgen = Instant::now();
            println!("\tgenerator: {:?}", tgen.duration_since(tstart));

            let part1 = part1(&input);
            let tpart1 = Instant::now();
            println!("Part 1: {}", part1);
            println!("\trunner: {:?}", tpart1.duration_since(tgen));

            let part2 = part2(&input);
            let tpart2 = Instant::now();
            println!("Part 2: {}", part2);
            println!("\trunner: {:?}", tpart2.duration_since(tpart1));

            println!();
        }
    };
}

fn main() {
    println!("Advent of Code 2020\n");

    solve!(include day01, "Day 1", "day1");

    solve!(include day02, "Day 2", "day2");
}