use std::collections::BTreeSet;

use crate::utils::{UnwrapUnchecked, lines};
use std::str::from_utf8_unchecked;

pub type Parsed = BTreeSet<u16>;
pub type Output1 = u32;
pub type Output2 = u32;

pub fn parse(input: &[u8]) -> Parsed {
    lines(input)
        .map(|line| unsafe { from_utf8_unchecked(line).parse().unwrap_unchecked() })
        .collect()
}

pub fn part1(input: &Parsed) -> Output1 {
    let opt = input.iter()
        .map(|&a| {
            let (b, _) = 2020_u16.overflowing_sub(a);
            (a, b)
        })
        .find(|(_, b)| input.contains(b));

    unsafe {
        let (a, b) = opt.unwrap_unchecked();
        a as u32 * b as u32
    }
}

pub fn part2(input: &Parsed) -> Output2 {
    let opt = input.iter()
        .flat_map(|&a| input.iter().map(move |&b| (a, b)))
        .filter_map(|(a, b)| 2020_u16.checked_sub(a + b).map(|c| (a, b, c)))
        .find(|(_, _, c)| input.contains(c));

    unsafe {
        let (a, b, c) = opt.unwrap_unchecked();
        a as u32 * b as u32 * c as u32
    }
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static [u8] = br#"1721
979
366
299
675
1456"#;

    #[test]
    fn test1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 514579)
    }

    #[test]
    fn test2() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), 241861950)
    }
}