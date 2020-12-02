type Input = String;
type Output1 = usize;
type Output2 = usize;

fn gen(input: &str) -> Input {
    todo!()
}

fn part1(_input: &Input) -> Output1 {
    0
}

fn part2(_input: &Input) -> Output2 {
    0
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static str = r#""#;

    #[test]
    fn test1() {
        let input = gen(INPUT);
        assert_eq!(part1(&input), 0)
    }

    #[test]
    fn test2() {
        let input = gen(INPUT);
        assert_eq!(part2(&input), 0)
    }
}