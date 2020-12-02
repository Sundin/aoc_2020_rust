use std::str::FromStr;
use std::num::ParseIntError;

pub struct PasswordPolicy {
    pub password: String,
    pub position1: usize,
    pub position2: usize,
    pub letter: char,
}

impl FromStr for PasswordPolicy {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input.split(":").collect::<Vec<&str>>();
        let password = split[1].trim();
        let first_half = split[0].split("-").collect::<Vec<&str>>();
        let min = first_half[0].parse::<usize>()?;
        let finale = first_half[1].split(" ").collect::<Vec<&str>>();
        let max = finale[0].parse::<usize>()?;
        let letter = finale[1].trim().chars().nth(0).unwrap();

        Ok(PasswordPolicy { password: password.to_string(), position1: min, position2: max, letter: letter })
    }
}

fn check_password_policy(password_policy: PasswordPolicy) -> bool {
    let occurences = password_policy.password.matches(password_policy.letter).count() as usize;
    occurences >= password_policy.position1 && occurences <= password_policy.position2
}

pub fn count_valid_password(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let password_policy = PasswordPolicy::from_str(line).unwrap();
        if check_password_policy(password_policy) {
            count += 1;
        }
    }
    count
}

fn check_password_policy_2(password_policy: PasswordPolicy) -> bool {
    let mut answer = false;
    if password_policy.password.chars().nth(password_policy.position1-1).unwrap() == password_policy.letter {
        answer = !answer;
    }
    if password_policy.password.chars().nth(password_policy.position2-1).unwrap() == password_policy.letter {
        answer = !answer;
    }
    answer
}

pub fn count_valid_occurences(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let password_policy = PasswordPolicy::from_str(line).unwrap();
        if check_password_policy_2(password_policy) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password_policy() {
        let p = PasswordPolicy { password: "abcde".to_string(), position1: 1, position2: 3, letter: 'a' };
        assert_eq!(true, check_password_policy(p));
        let p = PasswordPolicy { password: "ccccccccc".to_string(), position1: 2, position2: 9, letter: 'c' };
        assert_eq!(true, check_password_policy(p));
        let p = PasswordPolicy { password: "cdefg".to_string(), position1: 1, position2: 3, letter: 'b' };
        assert_eq!(false, check_password_policy(p));
    }

    #[test]
    fn test_occurence() {
        let p = PasswordPolicy { password: "abcde".to_string(), position1: 1, position2: 3, letter: 'a' };
        assert_eq!(true, check_password_policy_2(p));
        let p = PasswordPolicy { password: "ccccccccc".to_string(), position1: 2, position2: 9, letter: 'c' };
        assert_eq!(false, check_password_policy_2(p));
        let p = PasswordPolicy { password: "cdefg".to_string(), position1: 1, position2: 3, letter: 'b' };
        assert_eq!(false, check_password_policy_2(p));
    }
}