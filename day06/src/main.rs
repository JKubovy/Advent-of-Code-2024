use std::collections::{HashMap, HashSet};

use utils::{Coord, Direction, Turn};

#[derive(Debug, Clone, Copy)]
enum Cell {
    Obstacle,
    Empty,
}
type Grid = Vec<Vec<Cell>>;

const GUARD_TURN: Turn = Turn::Right;

fn try_get_cell(grid: &Grid, coord: Coord) -> Option<Cell> {
    grid.get(coord.y)
        .and_then(|line| line.get(coord.x).copied())
}

fn parse_input(input: &str) -> (Grid, Coord, Direction) {
    let mut start = None;
    let mut direction = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().fold(vec![], |mut data, (x, c)| {
                if let Ok(dir) = Direction::try_from(c) {
                    direction = Some(dir);
                    start = Some(Coord::from((x, y)))
                }
                data.push(if c == '#' {
                    Cell::Obstacle
                } else {
                    Cell::Empty
                });
                data
            })
        })
        .collect();
    (
        grid,
        start.expect("No start found"),
        direction.expect("No start found"),
    )
}

fn process_guard(
    grid: &Grid,
    mut guard_coord: Coord,
    mut guard_direction: Direction,
) -> (usize, bool) {
    let mut visited = HashMap::new();
    let mut loop_detected = false;
    loop {
        if is_coord_visited(&mut visited, guard_coord, guard_direction) {
            loop_detected = true;
            break;
        }
        visited.get_mut(&guard_coord).unwrap().push(guard_direction);
        if let Some(new_coord) = guard_coord + guard_direction {
            match try_get_cell(grid, new_coord) {
                Some(Cell::Obstacle) => guard_direction = guard_direction + GUARD_TURN,
                Some(Cell::Empty) => guard_coord = new_coord,
                None => break,
            }
            continue;
        }
        break;
    }
    (visited.keys().count(), loop_detected)
}

fn first_part(input: &str) -> usize {
    let (grid, guard_coord, guard_direction) = parse_input(input);
    process_guard(&grid, guard_coord, guard_direction).0
}

fn is_coord_visited(
    visited: &mut HashMap<Coord, Vec<Direction>>,
    guard_coord: Coord,
    guard_direction: Direction,
) -> bool {
    visited.entry(guard_coord).or_default();
    visited
        .get(&guard_coord)
        .unwrap()
        .contains(&guard_direction)
}

fn second_part(input: &str) -> usize {
    let (grid, guard_start_coord, guard_start_direction) = parse_input(input);
    let mut guard_coord = guard_start_coord;
    let mut guard_direction = guard_start_direction;
    let mut added_obstacles = HashSet::new();
    let mut visited = HashMap::new();
    loop {
        if is_coord_visited(&mut visited, guard_coord, guard_direction) {
            break;
        }
        visited.get_mut(&guard_coord).unwrap().push(guard_direction);
        if let Some(new_coord) = guard_coord + guard_direction {
            match try_get_cell(&grid, new_coord) {
                Some(Cell::Obstacle) => {
                    guard_direction = guard_direction + GUARD_TURN;
                }
                Some(Cell::Empty) => {
                    let mut grid = grid.clone();
                    grid[new_coord.y][new_coord.x] = Cell::Obstacle;
                    if let (_, true) =
                        process_guard(&grid, guard_start_coord, guard_start_direction)
                    {
                        added_obstacles.insert(new_coord);
                    }
                    guard_coord = new_coord
                }
                None => break,
            };
            continue;
        }
        break;
    }
    added_obstacles.remove(&guard_start_coord);
    added_obstacles.len()
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
        assert_eq!(result, 41);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 5516);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 6);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 2008);
    }
}
