#[derive(Clone, Copy)]
enum FlowDirection {
    Unknown,
    Increasing,
    Decreasing,
}

#[derive(Clone, Copy, PartialEq)]
enum SafeStatus {
    Safe,
    Unsafe,
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<u32>().expect("Can't parse number"))
                .collect()
        })
        .collect()
}

fn check_line(data: &[u32]) -> SafeStatus {
    data.iter()
        .fold(
            (None, FlowDirection::Unknown, SafeStatus::Safe),
            |(last, flow, status), &next_number| match (last, flow, status) {
                (_, _, SafeStatus::Unsafe) => (last, flow, status),
                (None, _, _) => (Some(next_number), FlowDirection::Unknown, SafeStatus::Safe),
                (Some(last), _, _) if last == next_number || last.abs_diff(next_number) > 3 => {
                    (Some(next_number), flow, SafeStatus::Unsafe)
                }
                (Some(last), FlowDirection::Unknown, _) => (
                    Some(next_number),
                    if last < next_number {
                        FlowDirection::Increasing
                    } else {
                        FlowDirection::Decreasing
                    },
                    SafeStatus::Safe,
                ),
                (Some(last), FlowDirection::Increasing, _) if last < next_number => {
                    (Some(next_number), flow, SafeStatus::Safe)
                }
                (Some(last), FlowDirection::Decreasing, _) if last > next_number => {
                    (Some(next_number), flow, SafeStatus::Safe)
                }
                _ => (Some(next_number), flow, SafeStatus::Unsafe),
            },
        )
        .2
}

fn first_part(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|line| check_line(line))
        .filter(|&s| s == SafeStatus::Safe)
        .count()
}

fn second_part(input: &str) -> usize {
    parse(input)
        .iter()
        .filter_map(|line| {
            if let SafeStatus::Safe = check_line(line) {
                return Some(());
            }
            for i in 0..line.len() {
                let mut line_without_one_record = line.clone();
                line_without_one_record.remove(i);
                if let SafeStatus::Safe = check_line(&line_without_one_record) {
                    return Some(());
                }
            }
            None
        })
        .count()
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
        assert_eq!(result, 2);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 224);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_second_part_custom() {
        let data = include_str!("../inputs/test_custom.txt");
        let result = second_part(data);
        assert_eq!(result, 3);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 293);
    }
}
