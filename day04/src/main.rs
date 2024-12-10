use utils::Coord;

trait GetChar {
    fn get_by_coord(&self, coord: Coord) -> Option<char>;
}

impl GetChar for Grid {
    fn get_by_coord(&self, coord: Coord) -> Option<char> {
        Some(*(self.get(coord.y)?.get(coord.x)?))
    }
}

type Grid = Vec<Vec<char>>;
type Direction = (isize, isize);

const ALL_DIRECTIONS: &[Direction] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const XMAS: &str = "XMAS";
const MAS: &str = "MAS";
const SAM: &str = "SAM";

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_possible_starts(grid: &Grid, start: char) -> Vec<Coord> {
    let mut result = vec![];
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &character)| {
            if character == start {
                result.push((x, y).into());
            }
        })
    });
    result
}

fn is_pattern_present(
    grid: &Grid,
    start: Coord,
    direction: &Direction,
    pattern: &str,
) -> Option<()> {
    for i in 0..pattern.len() {
        if grid.get_by_coord((start + (direction.0 * i as isize, direction.1 * i as isize))?)?
            != pattern.chars().nth(i)?
        {
            return None;
        }
    }
    Some(())
}

fn first_part(input: &str) -> usize {
    let grid = parse_input(input);
    get_possible_starts(&grid, 'X')
        .iter()
        .flat_map(|start| {
            ALL_DIRECTIONS
                .iter()
                .filter_map(|dir| is_pattern_present(&grid, *start, dir, XMAS))
        })
        .count()
}

fn is_x_mas(grid: &Grid, start: Coord) -> Option<()> {
    let top_left = start + (-1, -1);
    let bottom_left = start + (1, -1);
    is_pattern_present(grid, top_left?, &(1, 1), MAS).or(is_pattern_present(
        grid,
        top_left?,
        &(1, 1),
        SAM,
    ))?;
    is_pattern_present(grid, bottom_left?, &(-1, 1), MAS).or(is_pattern_present(
        grid,
        bottom_left?,
        &(-1, 1),
        SAM,
    ))?;
    Some(())
}

fn second_part(input: &str) -> usize {
    let grid = parse_input(input);
    get_possible_starts(&grid, 'A')
        .iter()
        .filter_map(|start| is_x_mas(&grid, *start))
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
        assert_eq!(result, 18);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 2517);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 9);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 1960);
    }
}
