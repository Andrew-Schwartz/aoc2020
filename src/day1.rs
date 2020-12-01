use std::collections::BTreeSet;
use std::hint::unreachable_unchecked;

type Input = BTreeSet<u16>;
type Output1 = u32;
type Output2 = u32;

#[aoc_generator(day1)]
fn gen(input: &str) -> Input {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> Output1 {
    let opt = input.iter()
        .map(|&a| {
            let (b, _) = 2020_u16.overflowing_sub(a);
            (a, b)
        })
        .find(|(_, b)| input.contains(b));

    if let Some((a, b)) = opt {
        a as u32 * b as u32
    } else {
        unsafe { unreachable_unchecked() }
    }
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> Output2 {
    let opt = input.iter()
        .flat_map(|&a| input.iter().map(move |&b| (a, b)))
        .filter_map(|(a, b)| 2020_u16.checked_sub(a + b).map(|c| (a, b, c)))
        .find(|(_, _, c)| input.contains(c));

    if let Some((a, b, c)) = opt {
        a as u32 * b as u32 * c as u32
    } else {
        unsafe { unreachable_unchecked() }
    }
}
