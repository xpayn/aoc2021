use std::{cmp::max, fmt::Debug, mem, num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
struct Coordinate {
    x: u16,
    y: u16,
}

pub struct Segment(Coordinate, Coordinate);

impl Coordinate {
    fn is_vertically_aligned_with(&self, other: &Coordinate) -> bool {
        self.x == other.x
    }

    fn is_horizontally_aligned_with(&self, other: &Coordinate) -> bool {
        self.y == other.y
    }
}

impl FromStr for Coordinate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<u16> = s
            .split(",")
            .map(|x| x.parse::<u16>())
            .collect::<Result<Vec<u16>, ParseIntError>>()?;
        Ok(Self {
            x: input[0],
            y: input[1],
        })
    }
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.0.is_vertically_aligned_with(&self.1)
    }

    fn is_horizontal(&self) -> bool {
        self.0.is_horizontally_aligned_with(&self.1)
    }

    fn sort(mut self) -> Self {
        if self.0.x > self.1.x || self.0.x == self.1.x && self.0.y > self.1.y {
            mem::swap(&mut self.0, &mut self.1);
        }
        self
    }
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s
            .split(" -> ")
            .map(|x| x.parse::<Coordinate>())
            .collect::<Result<Vec<Coordinate>, ParseIntError>>()?;
        Ok(Self(input[0], input[1]).sort())
    }
}

#[derive(Default)]
struct Grid {
    columns: Vec<Vec<u8>>,
    allow_diagonals: bool,
}

impl Grid {
    fn with_diagonals() -> Self {
        Self {
            columns: vec![],
            allow_diagonals: true,
        }
    }

    fn get_lines_len(&self) -> usize {
        self.columns.len()
    }

    fn get_columns_len(&self) -> usize {
        if self.columns.is_empty() {
            0
        } else {
            self.columns[0].len()
        }
    }

    fn grow_columns(&mut self, n: usize) {
        for c in self.columns.iter_mut() {
            c.extend(vec![0; n])
        }
    }

    fn grow_lines(&mut self, n: usize) {
        self.columns
            .extend(vec![vec![0; self.get_columns_len()]; n]);
    }

    fn fit_to_segment(&mut self, segment: &Segment) {
        {
            let max_x = max(segment.0.x, segment.1.x) as usize;
            if self.get_lines_len() < max_x + 1 {
                let nb_missing = max_x + 1 - self.get_lines_len();
                self.grow_lines(nb_missing);
            }
        }
        {
            let max_y = max(segment.0.y, segment.1.y) as usize;
            if self.get_columns_len() < max_y + 1 {
                let nb_missing = max_y + 1 - self.get_columns_len();
                self.grow_columns(nb_missing)
            }
        }
    }

    fn add_segment(&mut self, segment: &Segment) {
        self.fit_to_segment(segment);
        if segment.is_vertical() {
            let col = &mut self.columns[segment.0.x as usize];
            let start = segment.0.y as usize;
            let end = (segment.1.y + 1) as usize;
            for y in start..end {
                col[y] += 1;
            }
        } else if segment.is_horizontal() {
            let y = segment.0.y as usize;
            let start = segment.0.x as usize;
            let end = (segment.1.x + 1) as usize;
            for x in start..end {
                self.columns[x][y] += 1;
            }
        } else if self.allow_diagonals {
            let start = segment.0.x as usize;
            let end = (segment.1.x + 1) as usize;
            let direction: isize = if segment.1.y > segment.0.y { 1 } else { -1 };
            for (i, x) in (start..end).enumerate() {
                let y = (segment.0.y as isize + (i as isize) * direction) as usize;
                self.columns[x][y] += 1;
            }
        } else {
            unreachable!()
        }
    }

    fn count_overlap(&self) -> usize {
        self.columns
            .iter()
            .map(|column| column.iter().filter(|x| **x > 1).count())
            .sum()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for y in 0..self.get_columns_len() {
            for c in &self.columns {
                if c[y] == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", c[y])?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Segment> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Segment]) -> usize {
    let grid = input
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical())
        .fold(Grid::default(), |mut grid, s| {
            grid.add_segment(s);
            grid
        });

    grid.count_overlap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Segment]) -> usize {
    let grid = input.iter().fold(Grid::with_diagonals(), |mut grid, s| {
        grid.add_segment(s);
        grid
    });

    grid.count_overlap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn d5_part2() {
        let input = input_generator(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );
        assert_eq!(solve_part2(input.as_slice()), 12);
    }
}
