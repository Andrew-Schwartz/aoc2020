use crate::const_utils::stou;

// pub type Parsed<'a> = Vec<Passport<'a>>;
pub type Parsed<'a> = &'a [u8];
pub type Output1 = usize;
pub type Output2 = usize;

pub fn parse(input: &[u8]) -> Parsed {
    input
    // get rid of trailing newline
    // input.trim_end()
    //     .split("\n\n")
    //     .map(|group| {
    //         let mut pass = Passport::default();
    //         group.split(|c: char| c == ' ' || c == '\n')
    //             .for_each(|pair| pass.set(pair));
    //         pass
    //     })
    //     .collect()
}

// #[derive(Default, Debug)]
// pub struct Passport<'a> {
//     byr: Option<&'a str>,
//     iyr: Option<&'a str>,
//     eyr: Option<&'a str>,
//     hgt: Option<&'a str>,
//     hcl: Option<&'a str>,
//     ecl: Option<&'a str>,
//     pid: Option<&'a str>,
//     cid: Option<&'a str>,
// }
//
// impl<'a> Passport<'a> {
//     fn set(&mut self, pair: &'a str) {
//         let i = pair.find(':').unwrap();
//         let (key, value): (&str, &str) = pair.split_at(i);
//         let value: &str = value.trim_start_matches(':');
//         let var: &mut Option<&str> = match key {
//             "byr" => &mut self.byr,
//             "iyr" => &mut self.iyr,
//             "eyr" => &mut self.eyr,
//             "hgt" => &mut self.hgt,
//             "hcl" => &mut self.hcl,
//             "ecl" => &mut self.ecl,
//             "pid" => &mut self.pid,
//             "cid" => &mut self.cid,
//             _ => panic!("bad key: {:?}", key),
//         };
//         if let Some(str) = var {
//             panic!("key {:?} already had a value: {:?}", key, str)
//         }
//         *var = Some(value);
//     }
//
//     const fn all(&self) -> bool {
//         self.byr.is_some() &&
//             self.iyr.is_some() &&
//             self.eyr.is_some() &&
//             self.hgt.is_some() &&
//             self.hcl.is_some() &&
//             self.ecl.is_some() &&
//             self.pid.is_some()
//     }
// }

// pub fn part1(input: &Parsed) -> Output1 {
//     input.iter()
//         .filter(|pass| pass.all())
//         .count()
// }

// pub fn part2(input: &Parsed) -> Output2 {
//     input.iter()
//         .filter(|pass| pass.all())
//         .filter(|Passport { byr, iyr, eyr, hgt, hcl, ecl, pid, .. }| {
//             let byr = byr.unwrap().parse::<u16>().unwrap();
//             let iyr = iyr.unwrap().parse::<u16>().unwrap();
//             let eyr = eyr.unwrap().parse::<u16>().unwrap();
//             let hgt = hgt.unwrap();
//             let (num, units) = hgt.split_at(hgt.len() - 2);
//             let num = num.parse::<u8>();
//             let hgt = if let Ok(hgt) = num {
//                 match units {
//                     "cm" => 150 <= hgt && hgt <= 193,
//                     "in" => 59 <= hgt && hgt <= 76,
//                     _ => false,
//                 }
//             } else {
//                 false
//             };
//             let hcl = hcl.unwrap();
//             let hcl = hcl.starts_with('#') && hcl.len() == 7 && u32::from_str_radix(&hcl[1..hcl.len()], 16).is_ok();
//             let ecl = match ecl.unwrap() {
//                 "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
//                 _ => false,
//             };
//             let pid = pid.unwrap();
//             let pid = pid.len() == 9 && pid.parse::<u32>().is_ok();
//
//             1920 <= byr && byr <= 2002 &&
//                 2010 <= iyr && iyr <= 2020 &&
//                 2020 <= eyr && eyr <= 2030 &&
//                 hgt &&
//                 hcl &&
//                 ecl &&
//                 pid
//         })
//         .count()
// }

pub fn part1(_input: &Parsed) -> Output1 {
    const PART1: Output1 = cpart1(day_input!(bytes "04"));
    PART1
}

pub fn part2(_input: &Parsed) -> Output1 {
    const PART2: Output1 = cpart2(day_input!(bytes "04"));
    PART2
}

struct CPassport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

impl CPassport {
    const fn new() -> Self {
        Self {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
        }
    }

    const fn all(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }

    const fn get_num(&self, input: &[u8], i: &mut usize) -> Option<usize> {
        let mut num = [None; 4];

        let i0 = *i;
        loop {
            if input[*i] == b' ' || input[*i] == b'\n' { break; }
            if *i - i0 == 4 { return None; }
            num[*i - i0] = Some(input[*i]);
            *i += 1;
        }

        if *i - i0 == 4 {
            Some(stou(num))
        } else {
            None
        }
    }

    const fn byr(&mut self, input: &[u8], i: &mut usize) {
        if let Some(byr) = self.get_num(input, i) {
            self.byr = 1920 <= byr && byr <= 2002
        }
    }

    const fn iyr(&mut self, input: &[u8], i: &mut usize) {
        if let Some(iyr) = self.get_num(input, i) {
            self.iyr = 2010 <= iyr && iyr <= 2020
        }
    }

    const fn eyr(&mut self, input: &[u8], i: &mut usize) {
        if let Some(eyr) = self.get_num(input, i) {
            self.eyr = 2020 <= eyr && eyr <= 2030
        }
    }

    const fn hgt(&mut self, input: &[u8], i: &mut usize) {
        let mut hgt = [None; 3];
        let mut units = [0; 2];

        let i0 = *i;
        let mut i1 = usize::MAX;
        loop {
            match input[*i] {
                b' ' | b'\n' => break,
                digit if digit.is_ascii_digit() => {
                    if *i - i0 == 3 {
                        return;
                    }
                    hgt[*i - i0] = Some(digit);
                }
                letter if letter.is_ascii_alphabetic() => {
                    if i1 == usize::MAX { i1 = *i; };
                    if *i - i1 == 2 {
                        return;
                    }
                    units[*i - i1] = letter
                }
                _ => {
                    return;
                }
            }
            *i += 1;
        }

        let hgt = stou(hgt);
        self.hgt = match &units {
            b"cm" => 150 <= hgt && hgt <= 193,
            b"in" => 59 <= hgt && hgt <= 76,
            _ => false,
        }
    }

    const fn hcl(&mut self, input: &[u8], i: &mut usize) {
        if input[*i] != b'#' {
            loop {
                *i += 1;
                if input[*i] == b' ' || input[*i] == b'\n' { break }
            }
            return;
        }
        *i += 1;
        let i0 = *i;
        loop {
            match input[*i] {
                b' ' | b'\n' => break,
                bad if !bad.is_ascii_hexdigit() => {
                    loop {
                        *i += 1;
                        if input[*i] == b' ' || input[*i] == b'\n' { break }
                    }
                    return;
                },
                _ => {}
            };
            *i += 1;
        }
        self.hcl = *i - i0 == 6;
    }

    const fn ecl(&mut self, input: &[u8], i: &mut usize) {
        let mut ecl = [0; 3];
        let i0 = *i;
        loop {
            match input[*i] {
                b' ' | b'\n' => break,
                letter if letter.is_ascii_alphabetic() => {
                    if *i - i0 == 3 {
                        loop {
                            *i += 1;
                            if input[*i] == b' ' || input[*i] == b'\n' { break }
                        }
                        return;
                    }
                    ecl[*i - i0] = letter;
                }
                _ => {
                    loop {
                        *i += 1;
                        if input[*i] == b' ' || input[*i] == b'\n' { break }
                    }
                    return;
                }
            }
            *i += 1;
        }
        self.ecl = match &ecl {
            b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth" => true,
            _ => false,
        }
    }

    const fn pid(&mut self, input: &[u8], i: &mut usize) {
        let i0 = *i;
        loop {
            match input[*i] {
                b' ' | b'\n' => break,
                bad if !bad.is_ascii_digit() => {
                    loop {
                        *i += 1;
                        if input[*i] == b' ' || input[*i] == b'\n' { break }
                    }
                    return;
                },
                _ => {}
            }
            *i += 1;
        }
        self.pid = *i - i0 == 9;
    }
}

const fn cpart1(input: &[u8]) -> Output1 {
    let mut count = 0;
    let mut i = 0;
    let mut passport = CPassport::new();
    while i < input.len() {
        let mut key = [0u8; 3];
        let i0 = i;
        while i - i0 < 3 {
            key[(i - i0)] = input[i];
            i += 1;
        }
        // skip ':'
        i += 1;

        match &key {
            b"byr" => passport.byr = true,
            b"iyr" => passport.iyr = true,
            b"eyr" => passport.eyr = true,
            b"hgt" => passport.hgt = true,
            b"hcl" => passport.hcl = true,
            b"ecl" => passport.ecl = true,
            b"pid" => passport.pid = true,
            _ => {}
        };

        // skip data
        loop {
            i += 1;
            if input[i] == b' ' || input[i] == b'\n' { break; }
        }

        if input[i] == b'\n' {
            if i == input.len() - 1 || input[i + 1] == b'\n' {
                if passport.all() {
                    count += 1;
                }
                passport = CPassport::new();
                i += 1;
            }
        }
        i += 1;
    }
    count
}

const fn cpart2(input: &[u8]) -> Output2 {
    let mut count = 0;
    let mut i = 0;
    let mut passport = CPassport::new();
    while i < input.len() {
        let mut key = [0u8; 3];
        let i0 = i;
        while i - i0 < 3 {
            key[(i - i0)] = input[i];
            i += 1;
        }
        // skip ':'
        i += 1;

        match &key {
            b"byr" => passport.byr(input, &mut i),
            b"iyr" => passport.iyr(input, &mut i),
            b"eyr" => passport.eyr(input, &mut i),
            b"hgt" => passport.hgt(input, &mut i),
            b"hcl" => passport.hcl(input, &mut i),
            b"ecl" => passport.ecl(input, &mut i),
            b"pid" => passport.pid(input, &mut i),
            b"cid" => {
                loop {
                    if input[i] == b' ' || input[i] == b'\n' { break; }
                    i += 1;
                }
            }
            _ => {}
        };

        if input[i] == b'\n' {
            if i == input.len() - 1 || input[i + 1] == b'\n' {
                if passport.all() {
                    count += 1;
                }
                passport = CPassport::new();
                i += 1;
            }
        }
        i += 1;
    }
    count
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT1: &'static [u8] = br#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    #[test]
    fn test1() {
        assert_eq!(cpart1(INPUT1), 2)
    }

    const INPUT2A: &'static str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

    #[test]
    fn ctest2a() {
        assert_eq!(cpart2(INPUT2A.as_bytes()), 0)
    }

    const INPUT2B: &'static str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

    #[test]
    fn ctest2b() {
        assert_eq!(cpart2(INPUT2B.as_bytes()), 4)
    }
}