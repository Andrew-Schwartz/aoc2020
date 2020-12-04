use criterion::{black_box, Criterion, criterion_group, criterion_main};

use aoc2020::day02::*;

const INPUT: &'static [u8] = include_bytes!("../input/2020/day02.txt");

pub fn benchmark_gen(c: &mut Criterion) {
    const PARSED: Parsed = parse(INPUT);

    c.bench_function("Day 2 - Generator", |b| b.iter(|| PARSED));
}

pub fn benchmark_part1(c: &mut Criterion) {
    const PARSED: Parsed = parse(INPUT);
    const PART1: Output1 = part1(&PARSED);

    c.bench_function("Day 2 - Part 1", |b| b.iter(|| PART1));
}

pub fn benchmark_part2(c: &mut Criterion) {
    const PARSED: Parsed = parse(INPUT);
    const PART2: Output2 = part2(&PARSED);

    c.bench_function("Day 2 - Part 2", |b| b.iter(|| PART2));
}

criterion_group!(benches, benchmark_gen, benchmark_part1, benchmark_part2);
criterion_main!(benches);