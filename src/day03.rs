pub type Parsed<'a> = &'a [u8];
pub type Output1 = usize;
pub type Output2 = usize;

pub fn parse(_input: &[u8]) -> Parsed {
    const INPUT: Parsed = cparse(day_input!(bytes "03"));
    INPUT
}

const fn cparse(input: &'static [u8]) -> Parsed<'static> {
    input
}

pub fn part1(_input: &[u8]) -> Output1 {
    const PARSED: Parsed = cparse(day_input!(bytes "03"));
    const OUTPUT: Output1 = cpart1(&PARSED);
    OUTPUT
}

const fn cpart1(input: &[u8]) -> Output1 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let mut i = 0;
    let mut len = None;
    while i < input.len() {
        let b = input[i];

        if let Some(len) = len {
            if x == (3 * y) % len && b == b'#' {
                count += 1
            }
        }

        if b == b'\n' {
            len = Some(x);
            y += 1;
            x = 0;
        } else {
            x += 1;
        }

        i += 1;
    }
    count
}

pub fn part2(_input: &[u8]) -> Output2 {
    const PARSED: Parsed = cparse(day_input!(bytes "03"));
    const OUTPUT: Output2 = cpart2(&PARSED);
    OUTPUT
}

const fn cpart2(input: &[u8]) -> Output2 {
    let mut x = 0;
    let mut y = 0;
    let mut counts = (0, 0, 0, 0, 0);
    let mut i = 0;
    let mut len = None;
    while i < input.len() {
        let b = input[i];

        if let Some(len) = len {
            if x == y % len && b == b'#' {
                counts.0 += 1;
            }
            if x == (3 * y) % len && b == b'#' {
                counts.1 += 1;
            }
            if x == (5 * y) % len && b == b'#' {
                counts.2 += 1;
            }
            if x == (7 * y) % len && b == b'#' {
                counts.3 += 1;
            }
            if y % 2 == 0 && 2 * x == y % (len * 2) && b == b'#' {
                counts.4 += 1;
            }
        }

        if b == b'\n' {
            len = Some(x);
            y += 1;
            x = 0;
        } else {
            x += 1;
        }

        i += 1;
    }
    counts.0 * counts.1 * counts.2 * counts.3 * counts.4
}

// pub fn part1(input: &Input) -> Output1 {
//     input.iter()
//         .enumerate()
//         .skip(1)
//         .filter(|&(y, line)| {
//             let horiz = (3 * y) % line.len();
//             b'#' == line[horiz]
//         })
//         .count()
// }
//
// pub fn part2(input: &Input) -> Output2 {
//     input.iter()
//         .enumerate()
//         .skip(1)
//         .fold([0, 0, 0, 0, 0], |[a, b, c, d, e], (y, line)| {
//             [
//                 a + (b'#' == line[(1 * y) % line.len()]) as usize,
//                 b + (b'#' == line[(3 * y) % line.len()]) as usize,
//                 c + (b'#' == line[(5 * y) % line.len()]) as usize,
//                 d + (b'#' == line[(7 * y) % line.len()]) as usize,
//                 e + (y % 2 == 0 && b'#' == line[(1 * y / 2) % line.len()]) as usize,
//             ]
//         })
//         .iter()
//         .inspect(|it| println!("{:?}", it))
//         .product()
// }

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT: &'static [u8] = br#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;

    #[test]
    fn test1() {
        assert_eq!(cpart1(INPUT), 7)
    }

    #[test]
    fn test2() {
        assert_eq!(cpart2(INPUT), 336)
    }
}