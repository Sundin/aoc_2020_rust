
fn check_password(password: &str, min: i32, max: i32, letter: char) -> bool {
    let occurences = password.matches(letter).count() as i32;
    occurences >= min && occurences <= max
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
}