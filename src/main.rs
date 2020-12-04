#![feature(min_const_generics)]
#![feature(const_int_pow)]
#![feature(const_mut_refs)]

macro_rules! solve {
    (str $num:literal $parse:tt $p1:tt $p2:tt) => {
        {
            const INPUT: &'static str = include_str!(concat!("../input/2020/day", $num, ".txt"));
            solve!(@internal $num INPUT, $parse $p1 $p2);
        }
    };
    (file $num:literal) => {
        {
            let input = std::fs::read_to_string(concat!("input/2020/day", $num, ".txt")).unwrap();
            solve!(@internal $num input, let let let)
        }
    };
    (bytes $num:literal $parse:tt $p1:tt $p2:tt) => {
        {
            const INPUT: &'static [u8] = include_bytes!(concat!("../input/2020/day", $num, ".txt"));
            solve!(@internal $num INPUT, $parse $p1 $p2)
        }
    };
    (@internal $num:literal $input:expr, $parse:tt $p1:tt $p2:tt) => {
        {
            use std::time::Instant;
            paste::paste! {
                use [<day$num>]::*;
            }

            println!("Day {}", $num.strip_prefix('0').unwrap_or_else(|| $num));

            let tstart = Instant::now();
            #[allow(non_upper_case_globals)]
            $parse parsed: Parsed = parse(&$input);
            let tgen = Instant::now();
            println!("\tgenerator: {:?}", tgen.duration_since(tstart));
            let tgen = Instant::now();

            #[allow(non_upper_case_globals)]
            $p1 output1: Output1 = part1(&parsed);
            let tpart1 = Instant::now();
            println!("Part 1: {}", output1);
            println!("\trunner: {:?}", tpart1.duration_since(tgen));
            let tpart1 = Instant::now();

            #[allow(non_upper_case_globals)]
            $p2 output2: Output2 = part2(&parsed);
            let tpart2 = Instant::now();
            println!("Part 2: {}", output2);
            println!("\trunner: {:?}", tpart2.duration_since(tpart1));

            println!();
        }
    }
}

mod utils;
mod day01;
mod day02;
mod day03;

fn main() {
    println!("Advent of Code 2020\n");

    solve!(bytes "01" let let let);
    solve!(bytes "02" const const const);
    solve!(bytes "03" const const const);
}