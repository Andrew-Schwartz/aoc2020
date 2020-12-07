use criterion::{black_box, Criterion, criterion_group, criterion_main};

macro_rules! bench {
    (bytes $c:expr, $num:literal) => {
        {
            const INPUT: &'static [u8] = include_bytes!(concat!("../input/2020/day", $num, ".txt"));
            bench!(@internal $c, $num, INPUT);
        }
    };
    (str $c:expr, $num:literal) => {
        {
            const INPUT: &'static str = include_str!(concat!("../input/2020/day", $num, ".txt"));
            bench!(@internal $c, $num, INPUT);
        }
    };
    (@internal $c:expr, $num:literal, $input:expr) => {
        {
            paste::paste! {
                use aoc2020::[<day$num>]::*;
            }

            let trimmed = $num.trim_start_matches('0');
            $c.bench_function(&format!("Day {} - Generator", trimmed), |b| b.iter(|| parse(black_box(&$input))));

            let parsed = parse(&$input);
            $c.bench_function(&format!("Day {} - Part 1", trimmed), |b| b.iter(|| part1(black_box(&parsed))));
            $c.bench_function(&format!("Day {} - Part 2", trimmed), |b| b.iter(|| part2(black_box(&parsed))));
        }
    };
    ($c:expr => $($day:literal $form:tt $(,)?)+) => {
        $(
            bench!($form $c, $day);
        )+
    };
}

pub fn benchmark(c: &mut Criterion) {
    bench! { c =>
        "01" bytes,
        "02" bytes,
        "03" bytes,
        "04" bytes,
        "05" bytes,
        "06" bytes,
        "07" bytes,
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);