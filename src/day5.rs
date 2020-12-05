pub fn part_1(input: &str) -> i64 {
    0
}

pub fn part_2(input: &str) -> i64 {
    0
}

#[cfg(tests)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day5.txt".to_owned()).unwrap();
        assert_eq!(0, part_1(input: &str))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day5.txt".to_owned()).unwrap();
        assert_eq!(0, part_2(input: &str))
    }
}