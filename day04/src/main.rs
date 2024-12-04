#[derive(Clone, Copy, Debug)]
struct Coord {
    x: isize,
    y: isize,
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

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_possible_starts(grid: &Grid) -> Vec<Coord> {
    let mut result = vec![];
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &character)| {
            if character == 'X' {
                result.push(Coord {
                    x: x as isize,
                    y: y as isize,
                });
            }
        })
    });
    result
}

fn is_xmas_present(grid: &Grid, start: Coord, direction: &Direction) -> Option<()> {
    for i in 0..XMAS.len() {
        if *grid
            .get(usize::try_from((i as isize) * direction.0 + start.y).ok()?)?
            .get(usize::try_from((i as isize) * direction.1 + start.x).ok()?)?
            != XMAS.chars().nth(i)?
        {
            return None;
        }
    }
    Some(())
}

fn first_part(input: &str) -> usize {
    let grid = parse_input(input);
    get_possible_starts(&grid)
        .iter()
        .flat_map(|start| {
            ALL_DIRECTIONS
                .iter()
                .filter_map(|dir| is_xmas_present(&grid, *start, dir))
        })
        .count()
}

fn second_part(input: &str) -> usize {
    todo!();
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
        assert_eq!(result, todo!());
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, todo!());
    }
}
