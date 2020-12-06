pub type Parsed<'a> = &'a [u8];
pub type Output1 = u32;
pub type Output2 = u32;

pub fn parse(input: &[u8]) -> Parsed {
    input
}

pub fn part1(_input: &Parsed) -> Output1 {
    const OUTPUT: Output1 = cpart1(day_input!(bytes "06"));
    OUTPUT
}

pub fn part2(_input: &Parsed) -> Output2 {
    const OUTPUT: Output2 = cpart2(day_input!(bytes "06"));
    OUTPUT
}

const fn cpart1(input: &[u8]) -> Output1 {
    let mut count = 0;
    let mut answ: u32 = 0;
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'\n' => {
                if i == input.len() - 1 || input[i + 1] == b'\n' {
                    count += answ.count_ones();
                    answ = 0;
                    i += 1;
                }
            }
            letter => {
                let num = letter - b'a';
                answ |= 1 << num;
            }
        }

        i += 1;
    }
    count
}

const fn cpart2(input: &[u8]) -> Output1 {
    let mut count = 0;
    let mut person_answ: u32 = 0;
    let mut answ: u32 = 0b11_1111_1111_1111_1111_1111_1111;
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'\n' => {
                answ &= person_answ;
                person_answ = 0;
                if i == input.len() - 1 || input[i + 1] == b'\n' {
                    count += answ.count_ones();
                    answ = 0b11_1111_1111_1111_1111_1111_1111;
                    i += 1;
                }
            }
            letter => {
                let num = letter - b'a';
                person_answ |= 1 << num;
            }
        }

        i += 1;
    }
    count
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static [u8] = br#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;

    #[test]
    fn test1() {
        assert_eq!(cpart1(INPUT), 11)
    }

    #[test]
    fn test2() {
        assert_eq!(cpart2(INPUT), 6)
    }
}