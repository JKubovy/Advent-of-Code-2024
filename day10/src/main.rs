use std::collections::HashSet;

use utils::{Coord, Direction, StrParser};

type Grid = Vec<Vec<usize>>;

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn parse_map(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.parse_usize()).collect())
        .collect()
}

fn get_trailheads(grid: &Grid) -> Vec<Coord> {
    grid.iter()
        .enumerate()
        .fold(vec![], |mut trailheads, (y, line)| {
            line.iter().enumerate().for_each(|(x, &cell)| {
                if cell == 0 {
                    trailheads.push(Coord { x, y });
                }
            });
            trailheads
        })
}

fn get_height(grid: &Grid, coord: &Coord) -> Option<usize> {
    grid.get(coord.y)
        .and_then(|line| line.get(coord.x).copied())
}

fn get_available_peaks(grid: &Grid, trailhead: &Coord) -> Vec<Coord> {
    let mut peaks = Vec::new();
    let mut queue = vec![*trailhead];
    while let Some(cell) = queue.pop() {
        if let Some(height) = get_height(grid, &cell) {
            if height == 9 {
                peaks.push(cell);
                continue;
            }
            for &direction in ALL_DIRECTIONS {
                if let Some(next_cell) = cell + direction {
                    if let Some(next_height) = get_height(grid, &next_cell) {
                        if height + 1 == next_height {
                            queue.push(next_cell);
                        }
                    }
                }
            }
        }
    }
    peaks
}

fn first_part(input: &str) -> usize {
    let grid = parse_map(input);
    let trailheads = get_trailheads(&grid);
    trailheads
        .iter()
        .map(|trailhead| {
            let peaks = get_available_peaks(&grid, trailhead);
            let peaks: HashSet<&Coord> = HashSet::from_iter(peaks.iter());
            peaks.len()
        })
        .sum()
}

fn second_part(input: &str) -> usize {
    let grid = parse_map(input);
    let trailheads = get_trailheads(&grid);
    trailheads
        .iter()
        .map(|trailhead| get_available_peaks(&grid, trailhead).len())
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
        assert_eq!(result, 36);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 709);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 81);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 1326);
    }
}
