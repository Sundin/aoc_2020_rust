use itertools::Itertools;


pub fn part_1(input: &Vec<i32>, preamble: usize) -> i32 {
    for i in preamble..input.len() {
        let target_sum = *input.get(i).unwrap();
        println!("{}", target_sum);
        let sub_slice = &input[i-preamble..i].to_vec();
        let answer = find_sum(sub_slice, target_sum);
        if !answer {
            return target_sum;
        }
    }
    0
}

fn find_sum(input: &Vec<i32>, target_sum: i32) -> bool {
    println!("{:?}", input);

    //let mut v: Vec<i32> = &input;
    for vpair in input.into_iter().combinations(2) {
        let first = *vpair.first().unwrap();
        let second = *vpair.last().unwrap();
         let x = first + second;
         println!("{}, {}, {}", first, second, x);
         if x == target_sum {
             return true
         }
    }

    false
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
        let contents = files::get_file_contents("test_input/day9.txt".to_owned()).unwrap();
        let contents = files::as_integers(&contents);
        assert_eq!(127, part_1(&contents, 5));
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day9.txt".to_owned()).unwrap();
        let contents = files::as_integers(&contents);
        assert_eq!(0, part_2(&contents));
    }
}
