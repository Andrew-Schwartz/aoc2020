#![feature(min_const_generics)]
#![feature(const_int_pow)]
#![feature(const_mut_refs)]

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
            paste::paste! {
                use [<day$num>]::*;
            }

            println!("Day {}", $num.strip_prefix('0').unwrap_or_else(|| $num));

            let tstart = Instant::now();
            #[allow(non_upper_case_globals)]
            let parsed: Parsed = parse(&$input);
            let tgen = Instant::now();
            println!("\tgenerator: {:?}", tgen.duration_since(tstart));
            let tgen = Instant::now();

            #[allow(non_upper_case_globals)]
            let output1: Output1 = part1(&parsed);
            let tpart1 = Instant::now();
            println!("Part 1: {}", output1);
            println!("\trunner: {:?}", tpart1.duration_since(tgen));
            let tpart1 = Instant::now();

            #[allow(non_upper_case_globals)]
            let output2: Output2 = part2(&parsed);
            let tpart2 = Instant::now();
            println!("Part 2: {}", output2);
            println!("\trunner: {:?}", tpart2.duration_since(tpart1));

            println!();
        }
    };
    ($($day:literal $form:tt $(,)?)+) => {
        $(
            paste::paste! {
                mod [<day$day>];
            }
        )+

        fn main() {
            println!("Advent of Code 2020!\n");

            $(
                solve!($form $day);
            )+
        }
    };
}

#[macro_use]
mod utils;
mod const_utils;

solve! {
    "01" bytes,
    "02" bytes,
    "03" bytes,
    "04" bytes,
    "05" bytes,
    "06" bytes,
    "07" bytes,
}