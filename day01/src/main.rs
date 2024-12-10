use std::collections::HashMap;
use utils::StrParser;

fn get_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_ascii_whitespace();
            (
                nums.next().unwrap().parse_usize(),
                nums.next().unwrap().parse_usize(),
            )
        })
        .unzip()
}

fn first_part(input: &str) -> usize {
    let (mut left, mut right) = get_lists(input);
    left.sort();
    right.sort();
    left.iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>()
}

fn second_part(input: &str) -> usize {
    let (left, right) = get_lists(input);
    let right_counts = right.iter().fold(HashMap::new(), |mut data, n| {
        data.entry(n).or_default();
        *data.get_mut(n).unwrap() += 1;
        data
    });
    left.iter()
        .map(|n| n * right_counts.get(n).unwrap_or(&0))
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let first_part = first_part(input);
    println!("First part: {}", first_part);
    let second_part = second_part(input);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../inputs/test.txt");
        let result = first_part(data);
        assert_eq!(result, 11);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 2057374);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 31);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 23177084);
    }
}
