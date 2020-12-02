use criterion::{black_box, Criterion, criterion_group, criterion_main};

use aoc2020::day02::*;
use std::fs;

pub fn benchmark_gen(c: &mut Criterion) {
    let input = fs::read_to_string("input/2020/day2.txt").unwrap();

    c.bench_function("Day 2 - Generator", |b| b.iter(|| gen(black_box(&input))));
}

pub fn benchmark_part1(c: &mut Criterion) {
    let input = fs::read_to_string("input/2020/day2.txt").unwrap();
    let input = gen(&input);

    c.bench_function("Day 2 - Part 1", |b| b.iter(|| part1(black_box(&input))));
}

pub fn benchmark_part2(c: &mut Criterion) {
    let input = fs::read_to_string("input/2020/day2.txt").unwrap();
    let input = gen(&input);

    c.bench_function("Day 2 - Part 2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, benchmark_gen, benchmark_part1, benchmark_part2);
criterion_main!(benches);