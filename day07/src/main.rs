use utils::StrParser;

enum Operator {
    Add,
    Mul,
    Concat,
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (sum, numbers) = line.split_once(": ").unwrap();
            let sum = sum.parse_usize();
            (
                sum,
                numbers
                    .split_ascii_whitespace()
                    .map(|num| num.parse_usize())
                    .collect(),
            )
        })
        .collect()
}

fn can_add_operators(sum: usize, numbers: &[usize], operators: &[Operator]) -> bool {
    numbers
        .iter()
        .fold(vec![0], |results, &num| {
            results
                .iter()
                .flat_map(|res| {
                    operators
                        .iter()
                        .filter_map(|op| {
                            let op_result = match op {
                                Operator::Add => res + num,
                                Operator::Mul => res * num,
                                Operator::Concat => {
                                    res * 10usize.pow(num.checked_ilog10().unwrap_or(0) + 1) + num
                                }
                            };
                            if op_result <= sum {
                                return Some(op_result);
                            }
                            None
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .contains(&sum)
}

fn process(input: &str, operators: &[Operator]) -> usize {
    let data = parse_input(input);
    data.iter()
        .map(|(sum, numbers)| {
            if can_add_operators(*sum, numbers, operators) {
                return sum;
            }
            &0
        })
        .sum()
}

fn first_part(input: &str) -> usize {
    process(input, &[Operator::Add, Operator::Mul])
}

fn second_part(input: &str) -> usize {
    process(input, &[Operator::Add, Operator::Mul, Operator::Concat])
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
        assert_eq!(result, 3749);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 1038838357795);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 11387);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 254136560217241);
    }
}
