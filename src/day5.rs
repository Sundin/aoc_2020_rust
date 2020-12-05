use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub struct BoardingPass {
    pub binary: String,
    pub seat_id: i32,
}

impl FromStr for BoardingPass {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut seat_id: i32 = 0;
        let base: i32 = 2;
        for (i, c) in input.chars().rev().enumerate() {
            if c == 'R' || c == 'B' {
                seat_id += base.pow(i as u32);
            }
        }

        Ok(BoardingPass { binary: input.to_string(), seat_id: seat_id })
    }
}

pub fn part_1(input: &str) -> i64 {
    for line in input.lines() {
        let boarding_pass: BoardingPass = line.parse().unwrap();
    }
    0
}

pub fn part_2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day5.txt".to_owned()).unwrap();
        assert_eq!(0, part_1(&contents))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day5.txt".to_owned()).unwrap();
        assert_eq!(0, part_2(&contents))
    }
}