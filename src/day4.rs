
use lazy_static::lazy_static;
use regex::Regex;

use super::files;

fn is_valid(input: &str) -> bool {
    input.contains("byr:") &&
    input.contains("iyr:") &&
    input.contains("eyr:") &&
    input.contains("hgt:") &&
    input.contains("hcl:") &&
    input.contains("ecl:") &&
    input.contains("pid:")
}

fn is_valid_regex(input: &str) -> bool {
    birth_year_valid(&input) &&
    issue_year_valid(&input) &&
    expiration_year_valid(&input) &&
    height_valid(&input) &&
    hair_color_valid(&input) &&
    eye_color_valid(&input) &&
    passport_id_valid(&input)
}

fn birth_year_valid(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"byr:\d{4}\s"
            ).unwrap();
    }
    if !REGEX.find_iter(input).nth(0).is_some() {
        return false
    }
    let byr = REGEX.captures_iter(input).nth(0).unwrap();
    let byr = byr[0].trim().split(":").collect::<Vec<&str>>();
    let byr = files::to_number(byr[1]);
    byr >= 1920 && byr <= 2002
}

fn issue_year_valid(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"iyr:\d{4}\s"
            ).unwrap();
    }
    if !REGEX.find_iter(input).nth(0).is_some() {
        return false
    }
    let iyr = REGEX.captures_iter(input).nth(0).unwrap();
    let iyr = iyr[0].trim().split(":").collect::<Vec<&str>>();
    let iyr = files::to_number(iyr[1]);
    iyr >= 2010 && iyr <= 2020
}

fn expiration_year_valid(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"eyr:\d{4}\s"
            ).unwrap();
    }
    if !REGEX.find_iter(input).nth(0).is_some() {
        return false
    }
    let eyr = REGEX.captures_iter(input).nth(0).unwrap();
    let eyr = eyr[0].trim().split(":").collect::<Vec<&str>>();
    let eyr = files::to_number(eyr[1]);
    eyr >= 2020 && eyr <= 2030
}

fn height_valid(input: &str) -> bool {
    valid_centimeters(&input) || valid_inches(&input)
}

fn valid_centimeters(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"hgt:(\d\d\d)(cm)\s"
            ).unwrap();
    }
    if !REGEX.find_iter(input).nth(0).is_some() {
        return false
    }
    let cm = REGEX.captures_iter(input).nth(0).unwrap();
    let cm = cm[1].trim().split(":").collect::<Vec<&str>>();
    let cm = files::to_number(cm[0]);
    cm >= 150 && cm <= 193
}

fn valid_inches(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"hgt:(\d\d)(in)\s"
            ).unwrap();
    }
    if !REGEX.find_iter(input).nth(0).is_some() {
        return false
    }
    let inches = REGEX.captures_iter(input).nth(0).unwrap();
    let inches = inches[1].trim().split(":").collect::<Vec<&str>>();
    let inches = files::to_number(inches[0]);
    inches >= 59 && inches <= 76
}

fn hair_color_valid(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"hcl:\#[\dabcdef]{6}\s"
            ).unwrap();
    }
    REGEX.find_iter(input).nth(0).is_some()
}

fn eye_color_valid(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\s"
            ).unwrap();
    }
    REGEX.find_iter(input).nth(0).is_some()
}

fn passport_id_valid(input: &str) -> bool {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"pid:\d{9}\s"
            ).unwrap();
    }
    REGEX.find_iter(input).nth(0).is_some()
}

pub fn part_1(input: &str) -> i64 {
    let mut count = 0;
    let mut passport = String::new();
    for line in input.lines() {
        if line == "" {
            if is_valid(&passport) {
                count += 1;
            }
            passport = String::new();
        } else {
            passport = String::new() + &passport + line + " ";
        }
    }
    if is_valid(&passport) {
        count += 1;
    }
    count
}

pub fn part_2(input: &str) -> i64 {
    let mut count = 0;
    let mut passport = String::new();
    for line in input.lines() {
        if line == "" {
            if is_valid_regex(&passport) {
                count += 1;
            }
            passport = String::new();
        } else {
            passport = String::new() + &passport + line + " ";
        }
    }
    if is_valid_regex(&passport) {
        count += 1;
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_is_valid() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert_eq!(true, is_valid(&input));

        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";
        assert_eq!(false, is_valid(&input));

        let input = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";
        assert_eq!(true, is_valid(&input));

        let input = "hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";
        assert_eq!(false, is_valid(&input));
    }

    #[test]
    fn test_pid() {
        let input = "pid:000000001 ";
        assert_eq!(true, passport_id_valid(&input));
        let input = "pid:0123456789 ";
        assert_eq!(false, passport_id_valid(&input));
    }

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day4.txt".to_owned()).unwrap();
        assert_eq!(10, part_1(&contents));
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day4.txt".to_owned()).unwrap();
        assert_eq!(6, part_2(&contents));
    }
}
