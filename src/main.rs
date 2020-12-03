use paste::paste;

mod utils;
mod day01;
mod day02;

macro_rules! solve {
    (str $num:literal) => {
        {
            const INPUT: &'static str = include_str!(concat!("../input/2020/day", $num, ".txt"));
            solve!(@internal $num INPUT);
        }
    };
    (file $num:literal) => {
        {
            let input = std::fs::read_to_string(concat!("input/2020/day", $num, ".txt")).unwrap();
            solve!(@internal $num input)
        }
    };
    (bytes $num:literal) => {
        {
            const INPUT: &'static [u8] = include_bytes!(concat!("../input/2020/day", $num, ".txt"));
            solve!(@internal $num INPUT)
        }
    };
    (@internal $num:literal $input:expr) => {
        {
            use std::time::Instant;
            paste! {
                use [<day$num>]::*;
            }

            println!("Day {}", $num.strip_prefix('0').unwrap_or_else(|| $num));

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
    }
}

fn main() {
    println!("Advent of Code 2020\n");

    solve!(str "01");
    solve!(str "02");
}