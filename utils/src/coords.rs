use std::ops::Add;

use crate::Direction;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Add<(isize, isize)> for Coord {
    type Output = Option<Coord>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Some(Coord {
            x: if rhs.0 >= 0 {
                self.x.checked_add(rhs.0 as usize)?
            } else {
                self.x.checked_sub(rhs.0.unsigned_abs())?
            },
            y: if rhs.1 >= 0 {
                self.y.checked_add(rhs.1 as usize)?
            } else {
                self.y.checked_sub(rhs.1.unsigned_abs())?
            },
        })
    }
}

impl Add<Direction> for Coord {
    type Output = Option<Coord>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => self + (0, -1),
            Direction::Right => self + (1, 0),
            Direction::Down => self + (0, 1),
            Direction::Left => self + (-1, 0),
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}
