use utils::StrParser;

struct Rules {
    data: Vec<(usize, usize)>,
}

impl Rules {
    fn new() -> Self {
        Rules { data: vec![] }
    }
}

fn parse_input(input: &str) -> (Rules, Vec<Vec<usize>>) {
    let result = input.lines().fold(
        (Rules::new(), vec![], true),
        |(mut rules, mut lines, parsing_rules), line| {
            if line.is_empty() {
                return (rules, lines, false);
            }
            if parsing_rules {
                let (a, b) = line
                    .split_once('|')
                    .map(|(a, b)| (a.parse_usize(), b.parse_usize()))
                    .unwrap();
                rules.data.push((a, b));
            } else {
                lines.push(line.split(',').map(|a| a.parse_usize()).collect());
            }
            (rules, lines, parsing_rules)
        },
    );
    (result.0, result.1)
}

fn check_line_rules(rules: &Rules, line: &Vec<usize>) -> bool {
    let mut seen = vec![];
    let mut all_rules_ok = true;
    line.iter().for_each(|n| {
        if !seen.iter().all(|&seen_number| {
            rules
                .data
                .iter()
                .filter(|(a, b)| b == seen_number)
                .all(|(a, _)| a != n)
        }) {
            all_rules_ok = false
        }
        seen.push(n);
    });
    all_rules_ok
}

fn get_middle(line: &Vec<usize>) -> usize {
    let lenght = line.len();
    *line.get(lenght.wrapping_div(2)).unwrap()
}

fn fix_line(rules: &Rules, line: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    let mut rules = rules
        .data
        .iter()
        .filter(|(a, b)| line.contains(a) && line.contains(b))
        .collect::<Vec<_>>();
    // what if there is some numbers without rules where to put them?
    for _ in 0..line.len() {
        let next = rules
            .iter()
            .find(|(a, _)| !rules.iter().any(|(_, x)| a == x))
            .map(|a| a.0);
        if let Some(next) = next {
            rules.retain(|(a, _)| *a != next);
            result.push(next);
        }
    }
    result
}

fn first_part(input: &str) -> usize {
    let (rules, data) = parse_input(input);
    data.iter()
        .filter(|&line| check_line_rules(&rules, line))
        .map(get_middle)
        .sum()
}

fn second_part(input: &str) -> usize {
    let (rules, data) = parse_input(input);
    data.iter()
        .filter_map(|line| {
            if !check_line_rules(&rules, line) {
                return Some(get_middle(&fix_line(&rules, line)));
            }
            None
        })
        .sum()
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
        assert_eq!(result, 143);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 5087);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 123);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 4971);
    }
}
