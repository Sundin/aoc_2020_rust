fn check_password(password: &str, min: u32, max: u32, letter: char) -> bool {
    let occurences = password.matches(letter).count() as u32;
    occurences >= min && occurences <= max
}

fn parse_and_check_password(input: &str) -> bool {
    let split = input.split(":").collect::<Vec<&str>>();
    let password = split[1].trim();
    let first_half = split[0].split("-").collect::<Vec<&str>>();
    let min = first_half[0].parse::<u32>().unwrap();
    let finale = first_half[1].split(" ").collect::<Vec<&str>>();
    let max = finale[0].parse::<u32>().unwrap();
    let letter = finale[1].trim().chars().nth(0).unwrap();
    println!("input: {}, min: {}, max: {}, letter: {}, password: {}", input, min, max, letter, password);
    check_password(password, min, max, letter)
}

pub fn count_valid_password(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        if parse_and_check_password(line) {
            count += 1;
        }
    }
    count
}

fn check_occurence(password: &str, position1: usize, position2: usize, letter: char) -> bool {
    let mut answer = false;
    if password.chars().nth(position1-1).unwrap() == letter {
        answer = !answer;
    }
    if password.chars().nth(position2-1).unwrap() == letter {
        answer = !answer;
    }
    answer
}

pub fn count_valid_occurences(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        if parse_and_check_occurences(line) {
            count += 1;
        }
    }
    count
}

fn parse_and_check_occurences(input: &str) -> bool {
    let split = input.split(":").collect::<Vec<&str>>();
    let password = split[1].trim();
    let first_half = split[0].split("-").collect::<Vec<&str>>();
    let min = first_half[0].parse::<usize>().unwrap();
    let finale = first_half[1].split(" ").collect::<Vec<&str>>();
    let max = finale[0].parse::<usize>().unwrap();
    let letter = finale[1].trim().chars().nth(0).unwrap();
    println!("input: {}, min: {}, max: {}, letter: {}, password: {}", input, min, max, letter, password);
    check_occurence(password, min, max, letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password() {
        assert_eq!(true, check_password("abcde", 1, 3, 'a'));
        assert_eq!(true, check_password("ccccccccc", 2, 9, 'c'));
        assert_eq!(false, check_password("cdefg", 1, 3, 'b'));
    }

    #[test]
    fn test_parse_and_check() {
        assert_eq!(true, parse_and_check_password("1-3 a: abcde"));
        assert_eq!(true, parse_and_check_password("2-9 c: ccccccccc"));
        assert_eq!(false, parse_and_check_password("1-3 b: cdefg"));
    }

    #[test]
    fn test_occurence() {
        assert_eq!(true, check_occurence("abcde", 1, 3, 'a'));
        assert_eq!(false, check_occurence("cdefg", 1, 3, 'b'));
        assert_eq!(false, check_occurence("ccccccccc", 2, 9, 'c'));
    }

    #[test]
    fn test_parse_and_occur() {
        assert_eq!(true, parse_and_check_occurences("1-3 a: abcde"));
        assert_eq!(false, parse_and_check_occurences("2-9 c: ccccccccc"));
        assert_eq!(false, parse_and_check_occurences("1-3 b: cdefg"));
    }
}