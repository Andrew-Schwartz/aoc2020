pub type Parsed = String;
pub type Output1 = usize;
pub type Output2 = usize;

pub fn parse(input: &str) -> Input {
    todo!()
}

pub fn part1(_input: &Input) -> Output1 {
    todo!()
}

pub fn part2(_input: &Input) -> Output2 {
    todo!()
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static str = r#""#;

    #[test]
    fn test1() {
        let input = gen(INPUT);
        assert_eq!(part1(&input), todo!())
    }

    #[test]
    fn test2() {
        let input = gen(INPUT);
        assert_eq!(part2(&input), todo!())
    }
}