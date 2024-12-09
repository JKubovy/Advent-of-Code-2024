use std::collections::{HashMap, HashSet};

use utils::Coord;

type Size = (usize, usize);

fn parse_input(input: &str) -> (Size, HashMap<char, Vec<Coord>>) {
    let (mut max_x, mut max_y) = (0, 0);
    let antenas = input.lines().enumerate().fold(
        HashMap::new(),
        |mut antenas: HashMap<char, Vec<Coord>>, (y, line)| {
            max_y = y;
            line.chars().enumerate().for_each(|(x, cell)| {
                max_x = x;
                if cell != '.' {
                    antenas.entry(cell).or_default();
                    antenas.get_mut(&cell).unwrap().push(Coord { x, y });
                }
            });
            antenas
        },
    );
    ((max_x, max_y), antenas)
}

fn generate_antinodes(antenas: &[Coord], grid_size: Size) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();
    for first in 0..antenas.len() {
        for second in first + 1..antenas.len() {
            let first = *antenas.get(first).unwrap();
            let second = *antenas.get(second).unwrap();
            let diff = (
                second.x as isize - first.x as isize,
                second.y as isize - first.y as isize,
            );
            let first_antinode = first - diff;
            let second_antinode = second + diff;
            if first_antinode.is_some_and(|a| a.x <= grid_size.0 && a.y <= grid_size.1) {
                antinodes.insert(first_antinode.unwrap());
            }
            if second_antinode.is_some_and(|a| a.x <= grid_size.0 && a.y <= grid_size.1) {
                antinodes.insert(second_antinode.unwrap());
            }
        }
    }
    antinodes
}

fn generate_antinodes_with_harmonics(antenas: &[Coord], (max_x, max_y): Size) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();
    for first in 0..antenas.len() {
        for second in first + 1..antenas.len() {
            let mut first = *antenas.get(first).unwrap();
            let mut second = *antenas.get(second).unwrap();
            let diff = (
                second.x as isize - first.x as isize,
                second.y as isize - first.y as isize,
            );
            antinodes.insert(first);
            while let Some(a) = first - diff {
                if a.x > max_x || a.y > max_y {
                    break;
                }
                antinodes.insert(a);
                first = a;
            }
            antinodes.insert(second);
            while let Some(a) = second + diff {
                if a.x > max_x || a.y > max_y {
                    break;
                }
                antinodes.insert(a);
                second = a;
            }
        }
    }
    antinodes
}

fn first_part(input: &str) -> usize {
    let (grid_size, data) = parse_input(input);
    data.values()
        .fold(HashSet::new(), |mut antinodes, antenas| {
            antinodes.extend(generate_antinodes(antenas, grid_size));
            antinodes
        })
        .len()
}

fn second_part(input: &str) -> usize {
    let (grid_size, data) = parse_input(input);
    data.values()
        .fold(HashSet::new(), |mut antinodes, antenas| {
            antinodes.extend(generate_antinodes_with_harmonics(antenas, grid_size));
            antinodes
        })
        .len()
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
        assert_eq!(result, 14);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 252);
    }

    #[test]
    fn test_second_part() {
        let data: &str = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 34);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 839);
    }
}
