use std::str::FromStr;
use std::num::ParseIntError;
use super::files;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub operation: String,
    pub argument: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let operation = &input[0..3];
        let argument = &input[5..input.len()];
        let mut argument = files::to_number(argument);

        if (&input[4..5]).eq("-") {
            argument = 0 - argument;
        }


        Ok(Instruction { operation: operation.to_string(), argument: argument })
    }
}

pub fn part_1(input: &str) -> i32 {
    let mut program: Vec<Instruction> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut accumulator = 0;
    let mut index: i32 = 0;
    let mut visisted: Vec<i32> = Vec::new();

    loop {
        let instruction = program.get(index as usize).unwrap();

        if visisted.contains(&index) {
            return accumulator
        }
        visisted.push(index);

        // println!("Index: {}, Accumulator: {}, Operation: {}, argument: {}", index, accumulator, instruction.operation, instruction.argument);

        match instruction.operation.as_str() {
            "acc" => { 
                accumulator += instruction.argument
            },
            "jmp" => {
                index += instruction.argument;
                continue;
            },
            "nop" => {},
            _ => { println!("Not implemented") }
        }
        index += 1;
    }
}

pub fn part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day8.txt".to_owned()).unwrap();
        assert_eq!(5, part_1(&contents))
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day8.txt".to_owned()).unwrap();
        assert_eq!(0, part_2(&contents))
    }
}
