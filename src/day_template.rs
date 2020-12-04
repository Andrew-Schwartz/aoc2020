pub type Parsed = String;
pub type Output1 = usize;
pub type Output2 = usize;

pub fn parse(input: &str) -> Parsed {
    todo!()
}

pub fn part1(_input: &Parsed) -> Output1 {
    todo!()
}

pub fn part2(_input: &Parsed) -> Output2 {
    todo!()
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static str = r#""#;

    #[test]
    fn test1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), todo!())
    }

    #[test]
    fn test2() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), todo!())
    }
}