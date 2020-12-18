use regex::Regex;

lazy_static::lazy_static! {
    static ref EYE_MATCH: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref PID_MATCH: Regex = Regex::new("^\\d{9}$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Passport<'a> {
    pub byr: &'a str,
    pub iyr: &'a str,
    pub eyr: &'a str,
    pub hgt: &'a str,
    pub hcl: &'a str,
    pub ecl: &'a str,
    pub pid: &'a str,
    pub cid: Option<&'a str>,
}

#[derive(Default)]
struct Fields<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl Passport<'_> {
    pub fn is_strictly_valid(&self) -> bool {
        let is_number_between = |number: &str, low: u32, high: u32| {
            number.parse::<u32>()
                .map(|byr| byr >= low && byr <= high).unwrap_or(false)
        };

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let byr_valid = is_number_between(self.byr, 1920, 2002);

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let iyr_valid = is_number_between(self.iyr, 2010, 2020);

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let eyr_valid = is_number_between(self.eyr, 2020, 2030);

        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        let hgt_valid = if self.hgt.ends_with("cm") {
            is_number_between(self.hgt.trim_end_matches("cm"), 150, 193)
        } else if self.hgt.ends_with("in") {
            is_number_between(self.hgt.trim_end_matches("in"), 59, 76)
        } else { false };

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let hcl_valid = EYE_MATCH.is_match(self.hcl);

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let ecl_valid = match self.ecl {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        };

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let pid_valid = PID_MATCH.is_match(self.pid);

        // cid (Country ID) - ignored, missing or not.

        return byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid;
    }

    pub fn maybe_from_vec<'a>(fields: Vec<(&'a str, &'a str)>) -> Option<Passport<'a>> {
        let mut temp_fields = Fields::default();
        for (ident, value) in fields {
            match ident {
                "byr" => temp_fields.byr = Option::Some(value),
                "iyr" => temp_fields.iyr = Option::Some(value),
                "eyr" => temp_fields.eyr = Option::Some(value),
                "hgt" => temp_fields.hgt = Option::Some(value),
                "hcl" => temp_fields.hcl = Option::Some(value),
                "ecl" => temp_fields.ecl = Option::Some(value),
                "pid" => temp_fields.pid = Option::Some(value),
                "cid" => temp_fields.cid = Option::Some(value),
                _ => eprintln!("could not identify {} with value {}, skipping", ident, value)
            }
        }
        Passport::maybe_from_fields(temp_fields)
    }

    fn maybe_from_fields(fields: Fields) -> Option<Passport> {
        let byr = if let Some(a) = fields.byr { a } else { return Option::None; };
        let iyr = if let Some(a) = fields.iyr { a } else { return Option::None; };
        let eyr = if let Some(a) = fields.eyr { a } else { return Option::None; };
        let hgt = if let Some(a) = fields.hgt { a } else { return Option::None; };
        let hcl = if let Some(a) = fields.hcl { a } else { return Option::None; };
        let ecl = if let Some(a) = fields.ecl { a } else { return Option::None; };
        let pid = if let Some(a) = fields.pid { a } else { return Option::None; };
        let cid = fields.cid;
        Option::Some(Passport { byr, iyr, eyr, hgt, hcl, ecl, pid, cid })
    }
}

fn separate_pws(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn separate_pw_fields(input: &str) -> Vec<(&str, &str)> {
    input.split_ascii_whitespace().map(tokenize_pw_field).collect()
}

fn tokenize_pw_field(input: &str) -> (&str, &str) {
    let mut split = input.split(':');
    let identifier = split.next().expect(&format!("input {} has no first part", input));
    let value = split.next().expect(&format!("input {} has no second part", input));

    (identifier, value)
}

pub fn get_lenient_valid_pws_from_input(input: &str) -> impl Iterator<Item=Passport> {
    separate_pws(input).into_iter()
        .map(separate_pw_fields)
        .filter_map(Passport::maybe_from_vec)
}

pub fn get_strict_valid_pws_from_input(input: &str) -> impl Iterator<Item=Passport> {
    get_lenient_valid_pws_from_input(input)
        .filter(Passport::is_strictly_valid)
}

pub fn solve() -> (usize, usize) {
    let input = include_str!("./input");

    let valid_passports = get_lenient_valid_pws_from_input(&input);
    let strict_valid_passports = get_strict_valid_pws_from_input(&input);
    (valid_passports.count(), strict_valid_passports.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (264, 224))
    }

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
    const STRICT_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const STRICT_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";


    #[test]
    fn should_count_valid_pws() {
        let vec = get_lenient_valid_pws_from_input(INPUT);
        assert_eq!(vec.count(), 2)
    }

    #[test]
    fn should_count_strict_invalid_pws() {
        let vec = get_strict_valid_pws_from_input(STRICT_INVALID);
        assert_eq!(vec.count(), 0)
    }

    #[test]
    fn should_count_strict_valid_pws() {
        let vec = get_strict_valid_pws_from_input(STRICT_VALID);
        assert_eq!(vec.count(), 4)
    }
}