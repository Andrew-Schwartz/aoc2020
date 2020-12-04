use criterion::{Criterion, criterion_group, criterion_main};

use aoc2020::day03::*;

const INPUT: &'static [u8] = include_bytes!("../input/2020/day03.txt");

// this just in things get really fast when they're done at compile time :)
pub fn benchmark_gen(c: &mut Criterion) {
    const PARSED: Parsed = parse(INPUT);
    c.bench_function("Day 3 - Generator", |b| b.iter(|| PARSED));
}

pub fn benchmark_part1(c: &mut Criterion) {
    const PARSED: Parsed = parse(INPUT);
    const PART1: Output1 = part1(&PARSED);

    c.bench_function("Day 3 - Part 1", |b| b.iter(|| PART1));
}

pub fn benchmark_part2(c: &mut Criterion) {
    const PARSED: Parsed = parse(INPUT);
    const PART2: Output2 = part2(&PARSED);

    c.bench_function("Day 3 - Part 2", |b| b.iter(|| PART2));
}

criterion_group!(benches, benchmark_gen, benchmark_part1, benchmark_part2);
criterion_main!(benches);