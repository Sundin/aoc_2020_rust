use itertools::Itertools;

pub fn part_1(input: &Vec<i64>, preamble: usize) -> i64 {
    for i in preamble..input.len() {
        let target_sum = *input.get(i).unwrap();
        let sub_slice = &input[i-preamble..i].to_vec();
        let answer = find_sum(sub_slice, target_sum);
        if !answer {
            return target_sum;
        }
    }
    0
}

fn find_sum(input: &Vec<i64>, target_sum: i64) -> bool {
    for vpair in input.into_iter().combinations(2) {
        let first = *vpair.first().unwrap();
        let second = *vpair.last().unwrap();
        let x = first + second;
        if x == target_sum {
            return true
        }
    }

    false
}

pub fn part_2(input: &Vec<i64>, preamble: usize) -> i64 {
    let target_sum = part_1(&input, preamble);
    for (i, number) in input.iter().enumerate() {
        let mut contiguous: Vec<i64> = Vec::new();
        let mut current_sum: i64 = 0;
        let mut j = i;
        while current_sum < target_sum {
            contiguous.push(*input.get(j).unwrap());
            j += 1;
            current_sum = contiguous.iter().sum();
        }
        if current_sum == target_sum {
            contiguous.sort_by(|a, b| a.cmp(b));
            return contiguous.first().unwrap() + contiguous.last().unwrap();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::files;

    #[test]
    fn test_part_1() {
        let contents = files::get_file_contents("test_input/day9.txt".to_owned()).unwrap();
        let contents = files::as_integers_64(&contents);
        assert_eq!(127, part_1(&contents, 5));
    }

    #[test]
    fn test_part_2() {
        let contents = files::get_file_contents("test_input/day9.txt".to_owned()).unwrap();
        let contents = files::as_integers_64(&contents);
        assert_eq!(62, part_2(&contents, 5));
    }
}
