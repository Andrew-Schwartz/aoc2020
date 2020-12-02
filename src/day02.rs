use itertools::Itertools;

pub type Input<'a> = Vec<Password<'a>>;
pub type Output1 = usize;
pub type Output2 = usize;

#[derive(Debug)]
pub struct Password<'a> {
    deets: (u8, u8),
    char: char,
    pass: &'a str,
}

impl<'a> Password<'a> {
    fn valid1(&self) -> bool {
        let count = self.pass.chars()
            .filter(|&c| c == self.char)
            .count();
        let (l, u) = self.deets;
        l <= count as u8 && count as u8 <= u
    }

    fn valid2(&self) -> bool {
        let mut chars = self.pass.chars();
        let (fst, snd) = self.deets;
        let a = chars.nth(fst as usize - 1).unwrap();
        let b = chars.nth((snd - fst - 1) as _).unwrap();
        (a == self.char) ^ (b == self.char)
    }
}

pub fn gen(input: &str) -> Input {
    input.lines()
        .map(|l| {
            let (n, c, s) = l.split(' ').collect_tuple().unwrap();
            let deets = n.split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple().unwrap();

            Password {
                deets,
                char: c.chars().nth(0).unwrap(),
                pass: s,
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> Output1 {
    input.iter()
        .filter(|p| p.valid1())
        .count()
}

pub fn part2(input: &Input) -> Output2 {
    input.iter()
        .filter(|p| p.valid2())
        .count()
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn test1() {
        let input = gen(INPUT);
        assert_eq!(part1(&input), 2)
    }

    #[test]
    fn test2() {
        let input = gen(INPUT);
        assert_eq!(part2(&input), 1)
    }
}