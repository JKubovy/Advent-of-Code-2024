use std::collections::HashMap;

fn first_part(input: &str) -> i32 {
    let (left, right) = get_sorted_lists(input);
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
        .abs()
}

fn get_sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_ascii_whitespace();
            (
                nums.next()
                    .expect("Missing first number")
                    .parse::<i32>()
                    .expect("First is not a number"),
                nums.next()
                    .expect("Missing second number")
                    .parse::<i32>()
                    .expect("Second is not a number"),
            )
        })
        .unzip();
    left.sort();
    right.sort();
    (left, right)
}

fn second_part(input: &str) -> i32 {
    let (left, right) = get_sorted_lists(input);
    let right_counts = right
        .iter()
        .fold(
            (HashMap::new(), None, 0),
            |(mut data, last_seen, seen_count), n| {
                if last_seen.is_none() {
                    return (data, Some(n), 1);
                }
                if last_seen.unwrap() == n {
                    (data, last_seen, seen_count + 1)
                } else {
                    data.insert(last_seen.unwrap(), seen_count);
                    (data, Some(n), 1)
                }
            },
        )
        .0;
    left.iter()
        .map(|n| n * right_counts.get(n).unwrap_or(&0))
        .sum::<i32>()
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
