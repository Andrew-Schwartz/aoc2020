use State::*;
use crate::const_utils::stou;

pub type Parsed<'a> = &'a [u8];
pub type Output1 = usize;
pub type Output2 = usize;

// #[derive(Debug)]
// pub struct Password<'a> {
//     deets: (usize, usize),
//     char: u8,
//     pass: &'a [u8],
// }
//
// impl<'a> Password<'a> {
//     fn valid1(&self) -> bool {
//         let count = self.pass.iter()
//             .filter(|&&c| c == self.char)
//             .count();
//         let (l, u) = self.deets;
//         l <= count && count <= u
//     }
//
//     fn valid2(&self) -> bool {
//         let (a, b) = self.deets;
//         let a = self.pass[a - 1];
//         let b = self.pass[b - 1];
//         (a == self.char) ^ (b == self.char)
//     }
// }
//
// pub fn parse(input: &[u8]) -> Parsed {
//     lines(input)
//         .map(|l| unsafe {
//             let mut split = l.split(|&b| b == b' ');
//             let (deets, char, pass) = (
//                 {
//                     let mut n = split.next().unwrap_unchecked().split(|&b| b == b'-');
//                     let l = str::from_utf8_unchecked(n.next().unwrap_unchecked()).parse().unwrap_unchecked();
//                     let u = str::from_utf8_unchecked(n.next().unwrap_unchecked()).parse().unwrap_unchecked();
//                     (l, u)
//                 },
//                 split.next().unwrap_unchecked()[0],
//                 split.next().unwrap_unchecked(),
//             );
//
//             Password { deets, char, pass }
//         })
//         .collect()
// }
//
// pub fn part1(input: &Parsed) -> Output1 {
//     input.iter()
//         .filter(|p| p.valid1())
//         .count()
// }
//
// pub fn part2(input: &Parsed) -> Output2 {
//     input.iter()
//         .filter(|p| p.valid2())
//         .count()
// }

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    FirstNum,
    SecondNum,
    Key,
    Password,
}

const fn nums(sep: u8, i: &mut usize, input: &[u8]) -> usize {
    let mut num = [None; 3];
    let i0 = *i;
    loop {
        let c = input[*i];
        if c == sep {
            break;
        } else {
            num[*i - i0] = Some(c)
        }
        *i += 1;
    }
    stou(num)
}

pub fn parse(input: &[u8]) -> Parsed {
    input
}

pub fn part1(_input: &[u8]) -> Output1 {
    const OUTPUT: Output1 = cpart1(day_input!(bytes "02"));
    OUTPUT
}

pub fn part2(_input: &[u8]) -> Output2 {
    const OUTPUT: Output2 = cpart2(day_input!(bytes "02"));
    OUTPUT
}

const fn cpart1(input: &[u8]) -> Output1 {
    let mut count = 0;
    let mut state = FirstNum;
    let (mut l, mut u) = (0, 0);
    let mut key = 0;
    let mut i = 0;
    while i < input.len() {
        match state {
            FirstNum => {
                l = nums(b'-', &mut i, input);
                state = SecondNum;
            }
            SecondNum => {
                u = nums(b' ', &mut i, &input);
                state = Key;
            }
            Key => {
                key = input[i];
                // skip ": "
                i += 2;
                state = Password;
            }
            Password => {
                let mut num_match = 0;
                while input[i] != b'\n' {
                    if input[i] == key { num_match += 1; }
                    i += 1;
                }
                if l <= num_match && num_match <= u { count += 1 }
                state = FirstNum;
            }
        }

        i += 1;
    }
    count
}

const fn cpart2(input: &[u8]) -> Output2 {
    let mut count = 0;
    let mut state = FirstNum;
    let (mut l, mut u) = (0, 0);
    let mut key = 0;
    let mut i = 0;
    while i < input.len() {
        match state {
            FirstNum => {
                l = nums(b'-', &mut i, input);
                state = SecondNum;
            }
            SecondNum => {
                u = nums(b' ', &mut i, &input);
                state = Key;
            }
            Key => {
                key = input[i];
                // skip ": "
                i += 2;
                state = Password;
            }
            Password => {
                let i0 = i;
                let (mut a, mut b) = (false, false);
                while input[i] != b'\n' {
                    if i - i0 + 1 == l { a = input[i] == key };
                    if i - i0 + 1 == u { b = input[i] == key };

                    i += 1;
                }
                if a ^ b { count += 1 }
                state = FirstNum;
            }
        }

        i += 1;
    }
    count
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static [u8] = br#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;

    #[test]
    fn test1c() {
        assert_eq!(cpart1(INPUT), 2)
    }

    #[test]
    fn test2c() {
        assert_eq!(cpart2(INPUT), 1)
    }
}