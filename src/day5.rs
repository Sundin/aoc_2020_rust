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

pub fn part_1(input: &str) -> i32 {
    let boarding_passes = sort_boarding_passes(&input);
    boarding_passes.last().unwrap().seat_id
}

pub fn part_2(input: &str) -> i32 {
    let boarding_passes = sort_boarding_passes(&input);

    let mut previous_seat = 0;
    for bp in boarding_passes {
        if bp.seat_id == previous_seat + 2 {
            return bp.seat_id - 1;
        }
        previous_seat = bp.seat_id;
    }
    0
}

fn sort_boarding_passes(input: &str) -> Vec<BoardingPass> {
    let mut boarding_passes: Vec<BoardingPass> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();
        
    boarding_passes.sort_by(|a, b| a.seat_id.cmp(&b.seat_id));

    boarding_passes
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day5.txt".to_owned()).unwrap();
        assert_eq!(820, part_1(&contents))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day5.txt".to_owned()).unwrap();
        assert_eq!(566, part_2(&contents))
    }
}
