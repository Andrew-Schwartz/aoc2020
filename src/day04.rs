pub type Parsed<'a> = Vec<Passport<'a>>;
pub type Output1 = usize;
pub type Output2 = usize;

pub fn parse(input: &str) -> Parsed {
    // get rid of trailing newline
    input.trim_end()
        .split("\n\n")
        .map(|group| {
            let mut pass = Passport::default();
            group.split(|c: char| c == ' ' || c == '\n')
                .for_each(|pair| pass.set(pair));
            pass
        })
        .collect()
}

#[derive(Default, Debug)]
pub struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn set(&mut self, pair: &'a str) {
        let i = pair.find(':').unwrap();
        let (key, value): (&str, &str) = pair.split_at(i);
        let value: &str = value.trim_start_matches(':');
        let var: &mut Option<&str> = match key {
            "byr" => &mut self.byr,
            "iyr" => &mut self.iyr,
            "eyr" => &mut self.eyr,
            "hgt" => &mut self.hgt,
            "hcl" => &mut self.hcl,
            "ecl" => &mut self.ecl,
            "pid" => &mut self.pid,
            "cid" => &mut self.cid,
            _ => panic!("bad key: {:?}", key),
        };
        if let Some(str) = var {
            panic!("key {:?} already had a value: {:?}", key, str)
        }
        *var = Some(value);
    }

    const fn all(&self) -> bool {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }
}

pub fn part1(input: &Parsed) -> Output1 {
    input.iter()
        .filter(|pass| pass.all())
        .count()
}

pub fn part2(input: &Parsed) -> Output2 {
    input.iter()
        .filter(|pass| pass.all())
        .filter(|Passport { byr, iyr, eyr, hgt, hcl, ecl, pid, .. }| {
            let byr = byr.unwrap().parse::<u16>().unwrap();
            let iyr = iyr.unwrap().parse::<u16>().unwrap();
            let eyr = eyr.unwrap().parse::<u16>().unwrap();
            let hgt = hgt.unwrap();
            let (num, units) = hgt.split_at(hgt.len() - 2);
            let num = num.parse::<u8>();
            let hgt = if let Ok(hgt) = num {
                match units {
                    "cm" => 150 <= hgt && hgt <= 193,
                    "in" => 59 <= hgt && hgt <= 76,
                    _ => false,
                }
            } else {
                false
            };
            let hcl = hcl.unwrap();
            let hcl = hcl.starts_with('#') && hcl.len() == 7 && u32::from_str_radix(&hcl[1..hcl.len()], 16).is_ok();
            let ecl = match ecl.unwrap() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };
            let pid = pid.unwrap();
            let pid = pid.len() == 9 && pid.parse::<u32>().is_ok();

            1920 <= byr && byr <= 2002 &&
                2010 <= iyr && iyr <= 2020 &&
                2020 <= eyr && eyr <= 2030 &&
                hgt &&
                hcl &&
                ecl &&
                pid
        })
        .count()
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    const INPUT1: &'static str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
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
        let input = parse(INPUT1);
        assert_eq!(part1(&input), 2)
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
    fn test2a() {
        let input = parse(INPUT2A);
        assert_eq!(part2(&input), 0)
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
    fn test2b() {
        let input = parse(INPUT2B);
        assert_eq!(part2(&input), 4)
    }
}