use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub struct Passport {
    pub raw: String, 
}

impl FromStr for Passport {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let raw = input.to_owned();

        Ok(Passport { raw: raw })
    }
}

fn is_valid(input: &str) -> bool {
    input.contains("byr:") &&
    input.contains("iyr:") &&
    input.contains("eyr:") &&
    input.contains("hgt:") &&
    input.contains("hcl:") &&
    input.contains("ecl:") &&
    input.contains("pid:")
}

pub fn part_1(input: &str) -> i64 {
    let mut count = 0;
    let mut passport = String::new();
    for line in input.lines() {
        if line == "" {
            if is_valid(&passport) {
                count += 1;
                println!("valid passport {}: {}",count, passport);
            } else {
                println!("invalid passport: {}", passport);

            }
            passport = String::new();
        } else {
            passport = String::new() + &passport + line + " ";
        }
    }
    if is_valid(&passport) {
        count += 1;
        println!("valid passport {}: {}",count, passport);
    }
    count
}

pub fn part_2(input: &str) -> i64 {
    0
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
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day4.txt".to_owned()).unwrap();
        assert_eq!(2, part_1(&contents));
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day4.txt".to_owned()).unwrap();
        assert_eq!(0, part_2(&contents));
    }
}
