use criterion::{black_box, Criterion, criterion_group, criterion_main};

use aoc2020::day04::*;

const INPUT: &'static str = include_str!("../input/2020/day04.txt");

pub fn benchmark_gen(c: &mut Criterion) {
    c.bench_function("Day 4 - Generator", |b| b.iter(|| parse(black_box(INPUT))));
}

pub fn benchmark_part1(c: &mut Criterion) {
    let input = parse(INPUT);

    c.bench_function("Day 4 - Part 1", |b| b.iter(|| part1(black_box(&input))));
}

pub fn benchmark_part2(c: &mut Criterion) {
    let input = parse(INPUT);

    c.bench_function("Day 4 - Part 2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, benchmark_gen, benchmark_part1, benchmark_part2);
criterion_main!(benches);