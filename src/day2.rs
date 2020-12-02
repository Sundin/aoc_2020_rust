use substring::Substring;

fn check_password(password: &str, min: i32, max: i32, letter: char) -> bool {
    let occurences = password.matches(letter).count() as i32;
    occurences >= min && occurences <= max
}

fn parse_and_check_password(input: &str) -> bool {
    let min = input.chars().nth(0).unwrap() as i32;
    let max = input.chars().nth(2).unwrap() as i32;
    let letter = input.chars().nth(4).unwrap();
    let password = input.substring(7, input.len());
    check_password(password, min, max, letter)
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
}