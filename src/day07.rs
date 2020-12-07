use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use bstr::ByteSlice;
use itertools::Itertools;

pub type Parsed<'a> = &'a [u8];
pub type Output1 = usize;
pub type Output2 = usize;

#[derive(Debug, Copy, Clone)]
pub struct Bag<'a> {
    num: usize,
    color: &'a [u8],
}

impl<'a> Borrow<[u8]> for Bag<'a> {
    fn borrow(&self) -> &[u8] {
        self.color
    }
}

impl Hash for Bag<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

impl PartialEq for Bag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl Eq for Bag<'_> {}

pub fn parse(input: &[u8]) -> Parsed {
    input.trim()
}

pub fn part1(input: &Parsed) -> Output1 {
    let mut inner_to_outer = HashMap::new();
    for line in input.lines() {
        let bcont = b" bags contain";
        let i0 = line.find(bcont).unwrap();
        let outer = &line[..i0];
        line[i0 + bcont.len()..]
            .split(|&b| b == b',')
            .for_each(|inside| {
                let inside = inside.trim_start();
                if inside.find(b"no other").is_none() {
                    let i0 = inside.find_byte(b' ').unwrap();
                    let i1 = inside.find(b" bag").unwrap();
                    let color = &inside[i0 + 1..i1];
                    let entry = inner_to_outer.entry(color).or_insert(HashSet::new());
                    entry.insert(outer);
                }
            });
    }
    let mut holders = HashSet::new();
    let mut new = inner_to_outer.get(&b"shiny gold"[..]).unwrap().iter().collect_vec();
    while !new.is_empty() {
        let new_inner = new.remove(0);
        holders.insert(new_inner);
        if let Some(set) = inner_to_outer.get(new_inner) {
            new.extend(set);
        }
    }
    holders.len()
}

pub fn part2(input: &Parsed) -> Output2 {
    let outer_to_inner = input.lines()
        .map(|line| {
            let beginning = b" bags contain ";
            let i0 = line.find(beginning).unwrap();
            let inner = line[i0 + beginning.len()..]
                .split(|&b| b == b',')
                .filter_map(|bag| {
                    let bag = bag.trim_start();
                    if let Some(_) = bag.find("no other") {
                        None
                    } else {
                        let i0 = bag.find_byte(b' ').unwrap();
                        let num = unsafe { bag[..i0].to_str_unchecked() }.parse().unwrap();
                        let i1 = bag.find(b" bag").unwrap();
                        let color = &bag[i0 + 1..i1];
                        Some(Bag { num, color })
                    }
                })
                .collect();
            (&line[..i0], inner)
        })
        .collect();
    count(&b"shiny gold"[..], &outer_to_inner) - 1
}

fn count(current: &[u8], outer_to_inner: &HashMap<&[u8], HashSet<Bag>>) -> usize {
    let count = match outer_to_inner.get(current) {
        Some(inside) if !inside.is_empty() => {
            inside.iter()
                .map(|&Bag { num, color }| num * count(color, outer_to_inner))
                .sum::<usize>() + 1
        }
        Some(_) | None => {
            // just this one bag
            1
        }
    };
    count
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static [u8] = br#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

    #[test]
    fn test1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 4)
    }

    #[test]
    fn test2a() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), 32)
    }

    const INPUT2: &'static [u8] = b"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn test2b() {
        let input = parse(INPUT2);
        assert_eq!(part2(&input), 126)
    }
}