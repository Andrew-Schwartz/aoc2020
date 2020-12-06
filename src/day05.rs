pub type Parsed<'a> = &'a [u8];
pub type Output1 = usize;
pub type Output2 = usize;

pub fn parse(input: &[u8]) -> Parsed {
    input
}

pub fn part1(_input: &Parsed) -> Output1 {
    const OUTPUT: Output1 = cpart1(day_input!(bytes "05"));
    OUTPUT
}

pub fn part2(_input: &Parsed) -> Output2 {
    const OUTPUT: Output2 = cpart2(day_input!(bytes "05"));
    OUTPUT
}

const fn fill_seats(input: &[u8]) -> [bool; 8 * 128] {
    let mut seats = [false; 8 * 128];
    let mut i = 0;
    while i < input.len() {
        let (mut f, mut b) = (0, 127);
        let last_fb = i + 7;
        while i < last_fb {
            match input[i] {
                b'F' => {
                    b -= (b - f + 1) / 2;
                }
                b'B' => {
                    f += (b - f + 1) / 2;
                }
                _ => {}
            };
            i += 1;
        }
        let (mut l, mut r) = (0, 7);
        let last_lr = i + 3;
        while i < last_lr {
            match input[i] {
                b'L' => {
                    r -= (r - l + 1) / 2;
                }
                b'R' => {
                    l += (r - l + 1) / 2;
                }
                _ => {}
            };
            i += 1;
        }

        let id = l + 8 * f;
        seats[id] = true;

        i += 1;
    }
    seats
}

const fn cpart1(input: &[u8]) -> Output1 {
    let seats = fill_seats(input);
    let mut i = seats.len() - 1;
    loop {
        if seats[i] {
            return i;
        }
        i -= 1;
    }
}

const fn cpart2(input: &[u8]) -> Output2 {
    let seats = fill_seats(input);
    let mut i = 1;
    while i < seats.len() - 1 {
        if !seats[i] && seats[i - 1] && seats[i + 1] {
            return i;
        }
        i += 1;
    }
    123456789
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    #[test]
    fn test1a() {
        assert_eq!(cpart1(b"BFFFBBFRRR"), 567)
    }

    #[test]
    fn test1b() {
        assert_eq!(cpart1(b"FFFBBBFRRR"), 119)
    }

    #[test]
    fn test1c() {
        assert_eq!(cpart1(b"BBFFBBFRLL"), 820)
    }
}