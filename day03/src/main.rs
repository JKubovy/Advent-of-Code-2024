use regex::Regex;
use utils::StrParser;

fn first_part(input: &str) -> usize {
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    pattern
        .captures_iter(input)
        .map(|caps| caps.extract())
        .map(|(_, [num1, num2])| num1.parse_usize() * num2.parse_usize())
        .sum()
}

fn second_part(input: &str) -> usize {
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\)|don't\(\))").unwrap();
    pattern
        .captures_iter(input)
        .map(|caps| {
            (
                caps.get(1).map_or("", |m| m.as_str()),
                caps.get(2).map_or("", |m| m.as_str()),
                caps.get(3).map_or("", |m| m.as_str()),
            )
        })
        .fold((0, false), |(sum, skip), (num1, num2, command)| {
            match command {
                "do()" => return (sum, false),
                "don't()" => return (sum, true),
                _ => {}
            }
            if skip {
                return (sum, skip);
            }
            (
                sum + num1.parse::<usize>().expect("num1 is NOT number")
                    * num2.parse::<usize>().expect("num1 is NOT number"),
                skip,
            )
        })
        .0
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
        assert_eq!(result, 161);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 189527826);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test2.txt");
        let result = second_part(data);
        assert_eq!(result, 48);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 63013756);
    }
}
