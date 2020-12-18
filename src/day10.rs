use itertools::Itertools;

pub fn part_1(input: &Vec<i32>) -> i32 {
    let mut contiguous = input.clone();
    contiguous.push(0);
    contiguous.sort_by(|a, b| a.cmp(b));
    let device = contiguous.last().unwrap() + 3;
    contiguous.push(device);
    println!("{:?}", contiguous);

    let mut ones = 0;
    let mut threes = 0;
    for (a, b) in contiguous.into_iter().tuple_windows() {
        println!("{}, {}", a, b);
        if b - a == 1 {
            ones += 1;
        } else if b - a == 3 {
            threes += 1;
        }
    }
    ones * threes
}

pub fn part_2(input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day10.txt".to_owned()).unwrap();
        let contents = files::as_integers(&contents);
        assert_eq!(35, part_1(&contents));
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day10.txt".to_owned()).unwrap();
        let contents = files::as_integers(&contents);
        assert_eq!(0, part_2(&contents));
    }
}
